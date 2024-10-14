using System.Windows;
using System.Runtime.InteropServices;
using System.Reactive.Linq;

namespace WVVM;

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class CustomTag : ModelBase, IDisposable {
  
  public CustomTag() { 
    var task = Task.Run(async () => {
      while (true) {
        await Task.Delay(1000);
        Application.Current.Dispatcher.Invoke(() => Counter = Counter + 1);
      }
    });
  }
  
  private bool _disposed = false;

  public void Dispose() {
    Dispose(true);
    GC.SuppressFinalize(this);
  }

  protected virtual void Dispose(bool disposing) {
    if (!_disposed){
      if (disposing) {
        // _stream.Dispose();
      }
        // TODO: Free unmanaged resources here.
        // Note disposing has been done.
        _disposed = true;
    }
  }

  /* property */
  private int _Counter = 0;
  public int Counter {
    get => _Counter;
    set {
      if (value == _Counter) return;
      _Counter = value;
      OnPropertyChanged();
      // wv.CoreWebView2.ExecuteScriptAsync($$"""

      // """);
    }
  }

  public string JavaScript = $$"""
  (()=>{
    const { fromEvent, filter } = window.rxjs;
    class {{nameof(CustomTag)}} extends HTMLElement {
      #host = window.chrome.webview.hostObjects.{{nameof(CustomTag)}};
      #propertyChaneged$ = fromEvent(window.chrome.webview, 'message');
      #name = null;
      #count = 0;
      constructor() { 
        super(); 
        this.attachShadow({ mode: "open" });
      }

      /* 配置時初期化 */
      connectedCallback() { 
        this.#propertyChaneged$.pipe(filter(e => e.data.type === '{{nameof(CustomTag)}}PropertyChanged')).subscribe(async (e) => {
          const dst = await this.#host.Counter;
          this.#count = dst;
          const raisedEvent = new CustomEvent('countup', { 
            bubbles: false,
            cancelable: false,
            detail: this.#count
          });
          this.dispatchEvent(raisedEvent);
          this.#render();
        });
        this.#render();
      }
      disconnectedCallback() {
        console.log('Custom square element removed from page.');
        this.#propertyChaneged$.unsubscribe();
      }

      static observedAttributes = ["name"];    // 追加/削除/更新されたときに attributeChangedCallback が呼ばれる属性
      attributeChangedCallback(name, oldValue, newValue) {
        console.log('attributeChangedCallback', name);
        switch(name) {
          case 'name':
            this.#name = newValue;
            this.#render();
            break;
          default:
            break;
        }
      }

      // JavaScript でタグ作成したときの属性パラメータ
      get name() { return this.#name; }
      set name(value) { this.setAttribute('name', value); }

      #render() {
        this.shadowRoot.innerHTML = `<span>${this.#name} ${this.#count}</span>`;
      }
    }
    customElements.define("custom-tag", {{nameof(CustomTag)}});
  })();
  """;

}