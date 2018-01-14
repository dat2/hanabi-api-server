const server = require('http').createServer();

const io = require('socket.io')(server, {
  path: '/ws',
  serveClient: false,
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

createGame('1');

server.listen(process.env.PORT);
