( async ()=>{
  console.log("MainViewModel.js");
  const { model } = await window.Model;
  const { event : wvvmEvent } = window.Wvvm;
  
  await window.chrome.webview.hostObjects.Window.OpenDevToolsWindow();
  await model.addPropertyAsync();
  console.log(model.to_list())

  const viewmodel = {
    setup() {
      const { fromEvent, filter } = window.rxjs;
      window.Wvvm.init();
  
      const uri = window.Vue.ref('null');
      const title = window.Vue.ref('null');
      const date = window.Vue.ref('null');
      const path = window.Vue.computed(() => date.value + "_" + title.value);
  
      const wvNewWindowRequested$ = fromEvent(wvvmEvent, 'newWindowRequested');
      wvNewWindowRequested$.subscribe(async (e) => {
        uri.value = e.detail.Uri;
        window.Wvvm.dispatch({ type : "dragAndDrop", payload: [e.detail.Uri] })
        window.Wvvm.dispatch({ type : "message", payload: e.detail.Uri })
        window.Wvvm.dispatch({ type : "changeText", payload: e.detail.Uri })
      });
  
      const wvPropertyChaneged$ = fromEvent(wvvmEvent, 'propertyChanged');
      wvPropertyChaneged$.pipe(filter(e => e.detail.PropertyName === "text")).subscribe(async (e) => {
        console.log("propertychanged text", e);
      });
      wvPropertyChaneged$.pipe(filter(e => e.detail.PropertyName === "ProcessMessage")).subscribe(async (e) => {
        console.log("propertychanged ProcessMessage", e);
      });

      return {uri, title, date, path}
    }
  }
  const app = window.Vue.createApp(viewmodel).mount('#app');
})();

