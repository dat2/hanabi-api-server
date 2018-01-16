const express = require('express');
const socketIo = require('socket.io');
const http = require('http');
const uuidv4 = require('uuid/v4');

const app = express();
const server = http.createServer(app);

const io = socketIo(server, {
  path: '/ws',
  serveClient: false,
});

app.post('/api/games', function(req, res) {
  const newGameId = uuidv4();
  createGame(newGameId);
  res.json({ id: newGameId });
});

function createGame(gameId) {
  const nsp = io.of(`/games/${gameId}`);
  nsp.on('connection', socket => {
    socket.on('chat', payload => {
      socket.broadcast.emit('chat', payload);
    });
    socket.on('game', payload => {
      socket.broadcast.emit('game', payload);
    });
  });
}

server.listen(process.env.PORT);
