const server = require('http').createServer();

const io = require('socket.io')(server, {
  path: '/ws',
  serveClient: false,
});

io.on('connection', socket => {
  socket.on('message', payload => {
    socket.broadcast.emit('message', payload);
  });
});

server.listen(process.env.PORT);
