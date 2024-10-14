/******** impl from WPF code behind ********/
// const ARGS = {}



/******** squid class ********/
(function (global, factory) {
  // node                            -> module.exportsにfactory 
  // defineが関数かつdefine.amdがある -> defineにfactory
  // すべてfalse                     -> globalにglobalもしくはselfの代入
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define('squid', ['exports'], factory) :
  (factory((global.squid = {})));
}(this, (function (exports) {
  'use strict';

  /*
    squid.Window.OpenDevToolsWindow()
    squid.Window.ExecuteScriptAsync();
    squid.Window.ReadAllString();
    squid.Window.ZoomFactor
    squid.Window.Title
    squid.Window.Left
    squid.Window.Height
  */

  /*
    raisePropertyChanged()
    stateの追加
    cs側stateへのバイパス
  */

  class Squid {

    /* private field, property */

    #event = window.chrome.webview;
    get event() { return this.#event; };
    #window = window.chrome.webview.hostObjects.Window;
    get window() { return this.#window; };
    #pipe = window.chrome.webview.hostObjects.SquidPipe;
    get pipe() { return this.#pipe; };
    #mmf = window.chrome.webview.hostObjects.SquidMmf;
    get mmf() { return this.#mmf; };

    #reducer = {}
    
    /* public field */

    state = new StateBase();
    
    /* constructor */

    // asyncが使えない
    constructor() { }
    
    static async build() {

      const dst = new Squid();

      // const PropertyChanged$ = fromEvent(window.chrome.webview, 'message')
      //     .pipe(filter(e => e.data.type === "PropertyChanged"), map(e => e.data.data));
      // -> const PropertyChanged$ = fromEvent(squid.event, 'PropertyChanged');
      window.chrome.webview.addEventListener('message', (e) => {
        switch(e.data.type) {
          case 'propertyChanged':
          case 'newWindowRequested':
          case 'pipeMessageReceived':            
            const raisedEvent = new CustomEvent(e.data.type, { bubbles: false, cancelable: false, detail: e.data.data });
            window.chrome.webview.dispatchEvent(raisedEvent);
            break;
          default:
            break;
        }
      });

      await dst.state.addStateCs(window.chrome.webview.hostObjects.State);
      return dst;
    }

    /* method */

    addReducer(key, action) { this.#reducer[key] = action; }

    dispatch(json) {
      if(json.type in this.#reducer) { this.#reducer[json.type](json.payload); }
      window.chrome.webview.postMessage(JSON.stringify(json))
    }

    /*
      'navrequest' : D&D eventの捕捉
        eventの捕捉順は
        elementのdrop -> windowのdragover/drop -> CoreWebView2.NewWindowRequested -> windowのpop-up
        pop-upは不必要なのでNewWindowRequestedでe.Handled = true;代わりに
        elementのdrop -> windowのdragover/drop -> CoreWebView2.NewWindowRequested -> navrequest
    */

    drawFromMemoryMap = async function(id, bitshift) {
      const w = await this.#mmf.Width();
      const h = await this.#mmf.Height();

      const canv = document.getElementById(id);
      const ctx = canvas.getContext('2d', { willReadFrequently: true, alpha: false });
      const imageData = ctx.getImageData(0, 0, canv.width, canv.height);
      const src = await this.#mmf.ReadPixelsForJS(bitshift);
      // const clamp = new Uint8ClampedArray(src);
      imageData.data.set(src);
      ctx.putImageData(imageData, 0, 0);
    }

    removeEventListener() { throw new Error('NotImplementedError'); }

  }

  /* DDの禁止, 有効にした場合NewWindowRequestedにまでeventが伝播しnew windowがpopする */
  const disableWindowDropEvent = () => {
    window.addEventListener('dragover', function(e){ e.preventDefault(); }, false);
    window.addEventListener('drop', function(e){ e.preventDefault(); e.stopPropagation();}, false);
  }

  exports.Squid = Squid;
  exports.StateBase = StateBase;
  exports.disableWindowDropEvent = disableWindowDropEvent;

  Object.defineProperty(exports, '__esModule', { value: true }); // require対応

})));