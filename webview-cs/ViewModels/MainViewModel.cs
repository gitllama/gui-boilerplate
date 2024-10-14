using Microsoft.Extensions.DependencyInjection;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using Microsoft.Web.WebView2.Core;
using WVVM;

namespace WVVMSample;

public class MainWindowViewModel {
 
  public ReactivePropertySlim<string?> Path { get; set; }

  public ReactiveCommand<CoreWebView2NavigationCompletedEventArgs> NavigationCompleted { get; private set; }
  public ReactiveCommand<string> DispatchedCommand { get; private set; }

  public ReactiveCommand<string> BehaviorLoaded { get; private set; }
  public ReactivePropertySlim<Func<List<string>, Task<bool>>> RegistScript { get; set; }
  public ReactivePropertySlim<Func<string, Task<string>>> ExecuteScript { get; set; }

  public ReadOnlyReactiveProperty<string> ProcessMessage { get; set; }

  public MainWindowViewModel () {
    ConsoleEx.WriteInfo("constructing");
    var model = Ioc.Default.GetRequiredService<MainModel>();

    /*** Init ***/
    
    BehaviorLoaded = new ReactiveCommand<string>().WithSubscribe( /*async*/ (e) =>{
      // CoreWebView2InitializationCompletedだと、Helper Classの設定が間に合わないので、BehaviorLoadedを使用する
      ConsoleEx.WriteInfo($"BehaviorLoaded");
      var op = Ioc.Default.GetRequiredService<Args>();

      /*** regist HostObject / Controls ***/
      // wv.CoreWebView2.AddHostObjectToScript("SquidPipe", Ioc.Default.GetRequiredService<NamedPipe>());
      // wv.CoreWebView2.AddHostObjectToScript("SquidMmf", Ioc.Default.GetRequiredService<MemoryMap>());
      // wv.CoreWebView2.AddHostObjectToScript("statusLabel", this.statusLabel);
      // wv.CoreWebView2.AddHostObjectToScript("statusBar", this.statusBar);

      /******** navigate ********/
      var workingpath = System.IO.Path.Combine(System.IO.Directory.GetCurrentDirectory(), op.working);
      ConsoleEx.WriteNote($$"""
        navigate
          current : {{workingpath}}
          url : {{op.starturl}}
        """);
      System.IO.Directory.SetCurrentDirectory(workingpath);
      // Path.Value = op.starturl;
      // Path.Value = "Squid.Views.MainWindow.html";
    });    
    
    NavigationCompleted = new ReactiveCommand<CoreWebView2NavigationCompletedEventArgs>().WithSubscribe(async (e) =>{
      var message = $"NavigationCompleted : id = {e?.NavigationId} code = {e?.HttpStatusCode}";
      Dispatcher.dispatch($$"""{ "type" : "message", "payload" : "{{message}}" }""");
      await this.ExecuteScript!.Value("WVVMSample.Models.MainModel.js");
      await this.ExecuteScript!.Value("WVVMSample.ViewModels.MainViewModel.js");
      // await $"const ARGS = {model.args}".async_register_js(wv, false);
    });

    /*** Action ***/

    RegistScript = new ReactivePropertySlim<Func<List<string>, Task<bool>>>();

    ExecuteScript = new ReactivePropertySlim<Func<string, Task<string>>>();

    /*** INotifyPropertyChanged ***/

    model.ObserveProperty(o => o.Label).Subscribe(n=>{ ConsoleEx.WriteNote($$"""Label {{n}}"""); });
    model.ObserveProperty(o => o.ProcessMessage).Subscribe(n=>{ ConsoleEx.WriteNote($$"""ProcessMessage {{n}}"""); });
    this.ProcessMessage = model.ObserveProperty(x => x.ProcessMessage).ToReadOnlyReactiveProperty<string>();

    /*** INotifyPropertyChanged ***/
    this.Path = new ReactivePropertySlim<string?>(null);

    /***  ***/


    /*** v to m Event ***/
    DispatchedCommand = new ReactiveCommand<string>().WithSubscribe(e => Dispatcher.dispatch(e));

    /******** add dispatcher ********/
    // ConsoleEx.WriteNote("Init : add dispatcher");
    // await wv.CoreWebView2.ExecuteScriptAsync($$""" console.log("Init : add dispatcher") """);
      var hoge1 = Ioc.Default.GetRequiredKeyedService<ITest>("TestA");
      var hoge2 = Ioc.Default.GetRequiredKeyedService<ITest>("TestB");
      ConsoleEx.WriteNote(hoge1?.hoge);
      ConsoleEx.WriteNote(hoge2?.hoge);

  }

  struct EventMessager {
    public string type { get; set; }
    public object? data { get; set; }

    public EventMessager(string type, object? data){ this.type = type; this.data = data; }
    public string to_json() => System.Text.Json.JsonSerializer.Serialize(this);
  };

}
