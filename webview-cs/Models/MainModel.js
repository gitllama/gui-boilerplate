console.log("MainModel.js");
(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define('Model', ['exports'], factory) :
  (factory((global.Model = {})));
}(this, ( function (exports) {
  'use strict';
  const { ModelBase } = window.Wvvm;
  class Model extends ModelBase {

    #text = ""
    get text() { return this.#text }
    set text(value) {
      if(this.#text === value) return;
      this.#text = value
      this.raisePropertyChanged()
    }

    reduce(json) {
      switch(json.type) {
        case 'changeText':
          this.text = json.payload;
          break;
        default:
          break;
      }
    }
  }

  exports.model = new Model(); // Singleton
  Object.defineProperty(exports, '__esModule', { value: true });
})));