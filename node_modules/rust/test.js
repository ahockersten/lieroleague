var should = require('should'),
  rust = require('./index.js'),
  fs = require('fs'),
  async = require('async'),
  client = rust({config:'./app.config.test', args:'./vm.args.test'});

/*
 * test vars
 */
var host = '10.10.2.3', port = 9000, handoff_port = 9001, pb_port = 8000;

describe('API', function(){
  before(function(done){
    var files = ['./app.config', './vm.args'];
    var iterator = function(file, callback){
      var read = fs.createReadStream(file);
      var write = fs.createWriteStream(file + '.test');
      read.on('end', callback);
      read.on('error', callback);
      read.pipe(write);
    };
    async.each(files, iterator, function(err){
      if(err){
        throw err;
      }
      done();
    });
  });
  it('should set a host name', function(done){
    client.setHostName(host, function(err){
      should.equal(err, null);
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("{http, [ {\"" + host + '"'), -1);
        should.equal(blob.toString().indexOf("{https, [ {\"" + host + '"'), -1);
        done();
      });
    });
  });
  it('should set the http port', function(done){
    client.setHTTPPort(port, function(err){
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("{http, [ {\"" + host + '", ' + port), -1);
        should.equal(blob.toString().indexOf("{https, [ {\"" + host + '", ' + port), -1);
        done();
      });
    });
  });
  it('should set the handoff port', function(done){
    client.setHandoffPort(handoff_port, function(err){
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("handoff_port, " + handoff_port), -1);
        done();
      });
    });
  });
  it('should set the pb ip', function(done){
    client.setPBIP(host, function(err){
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("pb_ip, \"" + host + '"'), -1);
        done();
      });
    });
  });
  it('should set the pb port', function(done){
    client.setPBPort(pb_port, function(err){
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("pb_port, " + pb_port), -1);
        done();
      });
    });
  });
  it('should disable the pb interface', function(done){
    client.disablePB(function(err){
      fs.readFile('./app.config.test', function(err, blob){
        should.notEqual(blob.toString().indexOf("%% {pb_ip"), -1);
        should.equal(blob.toString().indexOf("/$1%% {pb_ip"), -1);
        done();
      });
    });
  });
  describe('Storage Backend', function(){
    it('should retrieve the backend storage type', function(done){
      client.backend.getType(function(err, name){
        name.should.equal('riak_kv_bitcask_backend');
        done();
      });
    });
    it('should set the type of the backend', function(done){
      client.backend.setType('riak_kv_eleveldb_backend', function(err){
        client.backend.getType(function(err, name){
          name.should.equal('riak_kv_eleveldb_backend');
          done();
        });
      });
    });
  });
  describe('riak search', function(){
    it('should see if search is enabled', function(done){
      client.getSearch(function(err, enabled){
        enabled.should.equal(false);
        done();
      });
    });
    it('should set search enabled', function(done){
      client.setSearch(true, function(err){
        client.getSearch(function(err, enabled){
          enabled.should.equal(true);
          done();
        });
      });
    });
  });
  describe('vm.args', function(){
    it('should get the node name', function(done){
      client.getNodeName(function(err, name){
        name.should.equal("riak@127.0.0.1");
        done();
      });
    });
    it('should set the node name', function(done){
      client.setNodeName('myriak@192.168.1.100', function(err){
        client.getNodeName(function(err, name){
          name.should.equal("myriak@192.168.1.100");
          done();
        });
      });
    });
    it('should get the cookie name', function(done){
      client.getCookieName(function(err, name){
        name.should.equal("riak");
        done();
      });
    });
    it('should set the cookie name', function(done){
      client.setCookieName("myriakCookie123", function(err){
        client.getCookieName(function(err, name){
          name.should.equal("myriakCookie123");
          done();
        });
      });
    });
  });
  after(function(done){
    var files = ['./app.config.test', './vm.args.test'];
    async.each(files, fs.unlink, function(err){
      if (err){
        throw err;
      }
      done();
    });
  });
});
