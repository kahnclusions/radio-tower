function main() {
  let root = window.document.getElementById("main");
  if (root === null) {
    console.error("[Dioxus] Could not find an element with ID 'main'");
  } else {
    window.ipc = new IPC(root);
  }
}

function log(msg) {
  "use strict";
  console.log(`[Dioxus] ${msg}`)
}

class IPC {
  constructor(root) {
    // connect to the websocket
    window.interpreter = new Interpreter(root);
    this.reconnecting = false;

    const overlayId = "rt-connection-status"
    const ping = () => {
      this.ws.send("__ping__");
    }

    const connect = () => {
      this.ws = new WebSocket(`${WS_ADDR}?initial_route=${window.location.pathname}`);
      this.ws.onopen = onopen;
      this.ws.onmessage = onmessage;
      this.ws.onclose = onclose;
    }

    const onopen = () => {
      const overlay = document.getElementById(overlayId)
      if (overlay) {
        document.getElementsByTagName("body")[0].removeChild(overlay)
      }
      if (this.reconnecting) {
        // Without the following, the app will be displayed twice:
        root.innerHTML = '';

        clearTimeout(this.reconnectDelaySetter);
        this.reconnectDelaySetter = undefined;
        this.reconnecting = false
      }
      // we ping every 30 seconds to keep the websocket alive
      setInterval(ping, 30000);
      this.ws.send(serializeIpcMessage("initialize"));
    };

    const showOverlay = () => {
      let overlay = document.getElementById(overlayId)
      if (!overlay) {
        overlay = document.createElement("div")
        overlay.setAttribute("id", overlayId)
        overlay.setAttribute("style", "position: fixed; top: 0; right: 0; left: 0; bottom: 0; background-color: #000000aa; display: flex; justify-content: center; align-items: center;")

        const overlayContent = document.createElement("div")
        overlayContent.setAttribute("id", "rt-connection-status-content")
        overlayContent.setAttribute("style", "color: white; font-size: 1.2rem; font-family: Silkscreen; margin: 3rem; font-weight: 500;")
        overlayContent.innerHTML = "<p style=\"margin-bottom: 1.5rem;\">Connection lost.</p><p>Please check your connection, and reload this tab if it doesn't come back online automatically.</p>"

        overlay.appendChild(overlayContent)
        this.overlay = overlay

        const body = document.getElementsByTagName("body")[0]
        body.appendChild(overlay)
      }
    }

    const onclose = (event) => {
      console.log("Closed")
      showOverlay()
      if (this.keepWsAliveIntervalId) {
        // Clear interval, so we don't ping the server again until we are reconnected:
        clearInterval(this.keepWsAliveIntervalId);
        this.keepWsAliveIntervalId = undefined;
      }

      if (!this.reconnecting) {
        log(`WebSocket closed – code: [${event.code}], reason: [${event.reason}]`);

        if (this.onDisconnect && this.onDisconnect.length > 0) {
          log("Executing client-side disconnection actions...");
          for (const action of this.onDisconnect) {
            try {
              if (action.type === "CallJsFn") {
                const fn = window[action.data];
                if (typeof fn !== "function") {
                  log(`ClientDisconnectAction Error: ${action.data} isn't a function`)
                  continue
                }
                fn()
              } else if (action.type === "SetAttribute") {
                const targets = document.querySelectorAll(action.data.selector);
                if (targets.length === 0) {
                  log(`ClientDisconnectAction Error: '${action.data.selector}' doesn't select any HTML elements`);
                  continue
                }
                for (const t of targets) {
                  t.setAttribute(action.data.name, action.data.value)
                }
              } else {
                log(`Unknown ClientDisconnectAction action: ${JSON.stringify(action)}`);
              }
            } catch (error) {
              console.error("[Error while executing `ClientDisconnectAction`]", error);
              continue;
            }
          }
        }

        // if (!DIOXUS_RECONNECT) {
        //   log("Configured to not reconnect => exit (page reload required to reconnect)")
        //   return
        // }

        log(`Attempting to reconnect`);
        log(`Note: WebSocket errors might be logged, until we are able to reconnect`);

        // Browsers log un-catchable errors to the developer console, when
        // network requests fail. Those errors will potentially be logged many
        // times until we successfully  reconnect. Unfortunately, there doesn't
        // seem to be a good way to avoid this. To not add to the noise with
        // our own log messages, we'll not log anything while reconnecting.

        this.reconnecting = true;

        // Setting the reconnection delay according to the settings in
        // `DIOXUS_RECONNECTION_DELAYS`:

        // const delays = [...DIOXUS_RECONNECT_DELAYS];
        // Start with 500ms delay.
        const delays = [[0, 512], [10 * 1000, 1024], [60 * 1000, 2048], [5 * 60 * 1000, 4096]];

        const setReconnectDelay = () => {
          // Keep the last delay value:
          if (delays.length === 0) return;

          const [duration, delay] = delays.shift();
          log(`Setting reconnection delay to ${delay}ms`)
          this.reconnectDelay = delay;
          this.reconnectDelaySetter = setTimeout(setReconnectDelay, duration);
        }

        setReconnectDelay();
      }

      // After a delay, try to reconnect:
      setTimeout(connect, this.reconnectDelay)
    };

    const onmessage = (event) => {
      // Ignore pongs
      if (event.data != "__pong__") {
        let msg = JSON.parse(event.data);
        if (msg.edits !== undefined && msg.onDisconnect !== undefined) {
          // The message we receive after (re-)connecting to the server
          this.onDisconnect = msg.onDisconnect;
          window.interpreter.handleEdits(msg.edits);
          return;
        }
        window.interpreter.handleEdits(msg);
      }
    };

    connect()
  }

  postMessage(msg) {
    if (this.ws?.readyState !== this.ws.OPEN) {
      return
    }
    this.ws.send(msg);
  }
}
