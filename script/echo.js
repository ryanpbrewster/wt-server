var PROTO_PATH = __dirname + '/../src/proto/echo.proto';
var grpc = require('grpc');
var protoLoader = require('@grpc/proto-loader');
// Suggested options for similarity to existing grpc.load behavior
var packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {keepCase: true,
     longs: String,
     enums: String,
     defaults: true,
     oneofs: true
    });
var echo = grpc.loadPackageDefinition(packageDefinition);

var stub = new echo.EchoService('localhost:50051', grpc.credentials.createInsecure());

stub.echo({ message: "t = " + Date.now() }, function(err, response) {
  if (err) {
    console.error("[ERROR] ", err);
  } else {
    console.log("received: ", response);
  }
});

