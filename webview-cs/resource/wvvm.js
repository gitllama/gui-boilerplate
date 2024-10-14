/******** extension method ********/ 
(()=>{
  Function.prototype.ex_method = function (name, func) {
    this.prototype[name] = func;
    return this;
  };
  String.ex_method('red',    function () { return `\u001b[31m${this}\u001b[0m`; });
  String.ex_method('green',  function () { return `\u001b[32m${this}\u001b[0m`; });
  String.ex_method('yellow', function () { return `\u001b[33m${this}\u001b[0m`; });
  String.ex_method('blue',   function () { return `\u001b[34m${this}\u001b[0m`; });
})();

/******** Wvvm module ********/
(function (global, factory) {
  // node                            -> module.exportsにfactory 
  // defineが関数かつdefine.amdがある -> defineにfactory
  // すべてfalse                     -> globalにglobalもしくはselfの代入
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define('Wvvm', ['exports'], factory) :
  (factory((global.Wvvm = {})));
}(this, (function (exports) {
  'use strict';

  class ModelBase {

    constructor() { }

    static async build(){
      let dst = new ModelBase();
      await dst.addPropertyAsync();
      return dst;
    }

    raisePropertyChanged(propertyName) {
      const stack = new Error().stack;
      const raisedEvent = new CustomEvent('propertyChanged', { 
        bubbles: false,
        cancelable: false,
        detail: { PropertyName : propertyName ?? stack.split('\n')[2].split(/\s+/)[3] }
      });
      window.chrome.webview.dispatchEvent(raisedEvent);
    }

    async addPropertyAsync() {
      let props = await window.chrome.webview.hostObjects.Model.GetPropertys();
      for(let key of props){
        Object.defineProperty(this, `${key}`, {
          get: async () => { return await window.chrome.webview.hostObjects.Model[key]; },
          set: (value) => { window.chrome.webview.hostObjects.Model[key] = value; }
        });
      }
    }

    addProperty(key, val){
      Object.defineProperty(this, "_" + key, {
        value: val,
        writable: true
      });
      Object.defineProperty(this, key, {
        get: () => { return this["_" + key] },
        set: (value) => {
          if(this["_" + key] === value) return;
          this["_" + key] = value  
          this.raisePropertyChanged(key)
        }
      });
    }

    to_list() { 
      return Object.getOwnPropertyDescriptors(this); // privateはでない
      // return Object.getOwnPropertyNames(this); 
    }

  }

  // const PropertyChanged$ = fromEvent(window.chrome.webview, 'message')
  //     .pipe(filter(e => e.data.type === "PropertyChanged"), map(e => e.data.data));
  // -> const PropertyChanged$ = fromEvent(squid.event, 'PropertyChanged');
  function init() {
    window.chrome.webview.addEventListener('message', (e) => {
      switch(e.data.type) {
        case 'propertyChanged':
        case 'newWindowRequested':
        case 'pipeMessageReceived':            
          const raisedEvent = new CustomEvent(e.data.type, { 
            bubbles: false,
            cancelable: false,
            detail: e.data.data
          });
          window.chrome.webview.dispatchEvent(raisedEvent);
          break;
        default:
          console.log("unknown event", e);
          break;
      }
    });
  }

  function dispatch(json) {
    window.Model.model.reduce(json);
    window.chrome.webview.postMessage(json) // C#側でjson化するのでJSON.stringify()不必要
    // await window.chrome.webview.hostObjects.Model.Reduce('{ "type" : "message", "payload" : "get url" }');
  }

  exports.init = init;
  exports.event = window.chrome.webview;
  exports.dispatch = dispatch;
  exports.ModelBase = ModelBase;

  Object.defineProperty(exports, '__esModule', { value: true }); // require対応
})));