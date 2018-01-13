const server = require('http').createServer();

const io = require('socket.io')(server, {
  path: '/ws',
  serveClient: false,
});

io.on('connection', socket => {
  socket.on('chat', payload => {
    socket.broadcast.emit('chat', payload);
  });
  socket.on('game', payload => {
    socket.broadcast.emit('game', payload);
  });
});

server.listen(process.env.PORT);
