var fs = require('fs'),
  spawn = require('child_process').spawn,
  backend = require('./backend'),
  replace = require('./replace'),
  search = require('./search');

module.exports = function(params){
  backend = backend(params);
  var rust = {
    setHostName:function(host, callback){
      var ops = {
        regex:/http, (.+)"/,
        value:'http, [ {"' + host + '"',
        file:params.config
      };
      replace(ops, callback);
    },
    getHostName:function(callback){
      search({
        regex:/http, (.+)"/,
        cleaner: [
          {clean: '[', cleanee: ''},
          {clean: '{', cleanee: ''},
          {clean: '"', cleanee: ''}
        ],
        trim: true,
        file:params.config
      }, callback);
    },
    setHTTPPort:function(port, callback){
      rust.getHostName(function(hostname){
        var ops = {
          regex:/http, (.+) }/,
          value:'http, [ {"' + hostname + '", ' + port + ' }',
          file:params.config
        };
        replace(ops, callback);
      });
    },
    setHandoffPort:function(port, callback){
      var ops = {
        regex:/handoff_port,(.+)}/,
        value:'handoff_port, ' + port + ' }',
        file:params.config
      };
      replace(ops, callback);
    },
    setPBIP:function(host, callback){
      replace({
        regex:/pb_ip,(.+)}/,
        value:'pb_ip, "' + host + '" }',
        file:params.config
      }, callback);
    },
    setPBPort:function(port, callback){
      replace({
        regex:/pb_port,(.+)}/,
        value:'pb_port, ' + port + ' }',
        file:params.config
      }, callback);
    },
    disablePB:function(callback){
      replace({
        regex:/{pb_ip/,
        value:'%% {pb_ip',
        file:params.config
      }, callback);
    },
    backend:backend,
    getSearch:function(callback){
      fs.readFile(params.config, function(err, blob){
        blob.toString().replace(/riak_search,\s+\[\s.+\n\s+{enabled,(.+)}/, function(r, search){
          if (typeof(search) === 'string'){
            search = search.trim() === 'true' ? true : false;
          }
          callback(null, search);
        });
      });
    },
    setSearch:function(enabled, callback){
      fs.readFile(params.config, function(err, blob){
        blob.toString().replace(/riak_search,(\s+\[\s.+\n\s+){enabled,(.+)}/, function(r, c, s){
          var buildString = r.replace(s.trim(), enabled);
          fs.writeFile(params.config, blob.toString().replace(r, buildString), function(err){
            callback(err);
          });
        });
      });
    },
    getNodeName:function(callback){
      fs.readFile(params.args, function(err, blob){
        blob.toString().replace(/-name(.+)/, function(r, name){
          callback(null, name.trim());
        });
      });
    },
    setNodeName:function(name, callback){
      replace({
        regex:/-name(.+)/,
        value:"-name " + name,
        file:params.args
      }, callback);
    },
    getCookieName:function(callback){
      fs.readFile(params.args, function(err, blob){
        blob.toString().replace(/-setcookie(.+)/, function(r, name){
          callback(null, name.trim());
        });
      });
    },
    setCookieName:function(name, callback){
      replace({
        regex:/-setcookie(.+)/,
        value:"-setcookie " + name,
        file:params.args
      }, callback);
    }
  };
  return rust;
};
