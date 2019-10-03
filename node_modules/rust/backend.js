var replace = require('./replace'),
    fs = require('fs');

module.exports = function(params){
  return {
    getType:function(callback){
      fs.readFile(params.config, function(err, blob){
        blob.toString().replace(/storage_backend,(.+)}/g, function(b, c){
          callback(null, c.trim());
        });
      });
    },
    setType:function(type, callback){
      replace({
        regex:/storage_backend,(.+)}/,
        value:'storage_backend, ' + type + ' }',
        file:params.config
      }, callback);
    }
  };
};

