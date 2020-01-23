////Websocket bs
var ip = location.hostname;
console.log(ip);
let socket = new WebSocket(`ws://${ip}:8765`);

socket.onopen = (event) => {
  document.getElementById('debug').innerHTML = '[open] Connection established';
};

socket.onmessage = (event) => {
  document.getElementById(`event`).innerHTML =
      `[message] Data received from server: ${event.data}`;
};

socket.onclose = (event) => {
  if (event.wasClean) {
    document.getElementById('debug').innerHTML =
        `[close] Connection closed cleanly, code=${event.code} reason=${
            event.reason}`;
  } else {
    // e.g. server process killed or network down
    // event.code is usually 1006 in this case
    document.getElementById('debug').innerHTML = '[close] Connection died';
  }
};

socket.onerror = (error) => {
  alert(`[error] ${error.message}`);
};


////////start of other stuff
let timers = {};
let buttons = {};

// loop through all <button>s and add event listeners to all of them!
// use the data-key as the key in the map
for (const button of document.querySelectorAll('button.zap-button')) {
  buttons[button.dataset.key] = button;
  // add the even listeners for unclicking
  for (const evt of ['mouseout', 'mouseup', 'touchend', 'touchcancel']) {
    button.addEventListener(evt, () => cancel(button));
  }
  // Cancel timers if the touch moves off the touch target.
  button.addEventListener('touchmove', (e) => {
    const touches = e.targetTouches;
    for (let i = 0; i < touches.length; i++) {
      const touch = touches.item(i);
      const elt = document.elementFromPoint(touch.clientX, touch.clientY);
      if (elt != button) {
        cancel(button);
        return;
      }
    }
  });
  // clicking sends the event and starts the timer to continue sending it(every
  // 100ms)!
  ['onclick', 'touchstart', 'mousedown'].forEach(
      evt => button.addEventListener(evt, (e) => {
        cancel(button);
        button.classList.add('btn-active');
        send(button);
        timers[button.dataset.key] = setInterval(() => send(button), 100);
        e.preventDefault();
      }));
}
// Id (unsigned int): Message Id
// DeviceIndex (unsigned int): Index of device
// Speeds (array): Vibration speeds
// Index (unsigned int): Index of vibration motor
// Speed (double): Vibration speed with a range of [0.0-1.0]
function send(button) {
  var data = JSON.stringify({
    'Id': Number(button.dataset.key),
    'DeviceIndex': Number(button.dataset.devind),
    'Speeds': [{
      'Index': Number(button.dataset.index),
      'Speed': Number(button.dataset.speed)
    }]

  });
  console.log(data);
  console.log(button.dataset.type)

  socket.send(data);  //`1${button.dataset.key}`);

  document.getElementById(`info${button.dataset.key}`).innerHTML =
      `sent for ${button.dataset.key}`;
}

function cancel(btn) {
  if (btn === null) {
    for (const button of Object.values(buttons)) {
      cancel(button);
    }
    return;
  }
  var data = JSON.stringify({'number': 1, 'state': 1, 'name': btn.dataset.key});
  console.log(data);
  // socket.send(data); //`1${button.dataset.key}`);

  // socket.send(`0${btn.dataset.key}`);
  btn.classList.remove('btn-active');
  if (timers[btn.dataset.key]) {
    document.getElementById(`info${btn.dataset.key}`).innerHTML =
        `canceled for ${btn.dataset.key}`
    clearInterval(timers[btn.dataset.key]);
    timers[btn.dataset.key] = null;
  }
}

document.addEventListener('blur', () => cancel(null));