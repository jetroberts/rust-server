const url = "ws://localhost:3000/api/ws";
const ws = new WebSocket(url);
ws.onmessage = (e) => {
  console.log(e.data);
};
