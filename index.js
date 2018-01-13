const server = require('http').createServer();

const io = require('socket.io')(server, {
  path: '/ws',
  serveClient: false,
});

io.on('connection', socket => {
  socket.on('message', payload => {
    socket.emit('message', payload);
  });
});

server.listen(process.env.PORT);
