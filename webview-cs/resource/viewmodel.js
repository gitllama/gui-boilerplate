(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define('squidvm', ['exports'], factory) :
  (factory((global.squidvm = {})));
}(this, (function (exports) {
  'use strict';

  const viewmodel = async (squid) => {
    const { fromEvent, range, filter, map, debounceTime, Observable } = window.rxjs;

    console.log("Observe webview message event")
    const wvPropertyChanged$ = fromEvent(squid.event, 'propertyChanged');
    wvPropertyChanged$.pipe(filter(e => e.detail.PropertyName === "Label")).subscribe(async (e) => {
      let val = await chrome.webview.hostObjects.State.Label;
      squid.Window.Title = `Squid - ${val}`;
    });
    wvPropertyChanged$.pipe(filter(e => e.detail.PropertyName === "Shift")).subscribe(async (e) => {
      let val = await chrome.webview.hostObjects.State.Shift;
      console.log("Shift", val);
      await squid.drawFromMemoryMap("canvas", val);
      squid.dispatch({ type : "d", payload: ["a"] })
    });
    wvPropertyChanged$.pipe(filter(e => e.detail.PropertyName === "keydown")).subscribe(async (e) => {
      console.log("PropertyChanged keydown event", e);
    });
  
    const wvNewWindowRequested$ = fromEvent(squid.event, 'newWindowRequested');
    wvNewWindowRequested$.subscribe(async (e) => console.log(e.detail.Uri) );
  
    const wvPipeMessageReceived$ = fromEvent(squid.event, 'pipeMessageReceived');
    wvPipeMessageReceived$.subscribe(async (e) => { console.log(e.detail) });

    console.log("Observe key / mouse event") 
    const mousemove$ = fromEvent(document.getElementById("wrapper"), 'mousemove');
    mousemove$.subscribe(async (event) => { 
      // .pipe(debounceTime(100))
      // https://cpplover.blogspot.com/2009/06/dom-level-3.html
      const rect = event.target.getBoundingClientRect();
      const x = Math.trunc(event.clientX - rect.left); // parseInt()はバグの温床
      const y = Math.trunc(event.clientY - rect.top);
      const val = await squid.Mmf.GetPixelInt(x, y);
      console.log(`${x} - ${y} : ${val}`)
      // squid.dispatch(`{"type" : "mousemove", "payload": ["${x}", "${y}"] }`);
    });
  
    const mousewheel$ = fromEvent(document.getElementById("wrapper"), 'mousewheel');
    mousewheel$.pipe(filter(e => e.shiftKey)).subscribe(async (event) => {
      event.preventDefault();
      if (event.wheelDelta > 0) {
        squid.dispatch({ type : "shift", payload : [1] })
      } else {
        squid.dispatch({ type : "shift", payload : [-1] })
      }
    });
    mousewheel$.pipe(filter(e => e.ctrlKey)).subscribe(async (event) => {
      // event.preventDefault();
      // stage.style.transform = `scale(${canvas_scale})`;
      console.log("zoom", await squid.Window.ZoomFactor);
    });
  
    const keyevent$ = fromEvent(window, 'keydown');
    keyevent$.pipe(filter(e => e.repeat === false)).subscribe(async (event) => {
      console.log(`keydown event ${event.shiftKey ? "shift + " : ""}${event.ctrlKey ? "ctrl + " : ""}${event.altKey ? "alt + " : ""}${event.key}`);
      squid.dispatch({ type : "keydown", "payload" : [event.key] })
    });

  }

  exports.viewmodel = viewmodel;
  Object.defineProperty(exports, '__esModule', { value: true }); // require対応

})));
