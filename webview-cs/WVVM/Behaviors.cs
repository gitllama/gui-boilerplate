
using System.Windows;
using Microsoft.Web.WebView2.Wpf;
using System.ComponentModel;
using Microsoft.Web.WebView2.Core;
using System.Windows.Input;
using System.Windows.Markup;
using System.Runtime.InteropServices;
using System.IO;
using System.Reactive.Linq;

namespace WVVM;

/*
  VM -> V の action

  ***prism : InteractionRequestTrigger***
    MessageAction : TriggerAction<DependencyObject>を呼ぶ
    [view]
    <p:InteractionRequestTrigger SourceObject="{Binding MessageRequest, Mode=OneTime}">
      <behaviors:MessageAction/>
    </p:InteractionRequestTrigger>
    [vm]
    InteractionRequest<Notification> _closeRequest
    _close2Request.Raise(n, new Action<Notification>(x => { }));
 
  ***TriggerAction***
    Propertyに同値が入ったときの処理がどうもしっくりこない
    [view]
    <wv2:WebView2>
      <i:Interaction.Triggers>
        <i:PropertyChangedTrigger Binding="{Binding Prop.Value}">
          <local:MessageAction />
        </i:PropertyChangedTrigger>
      </i:Interaction.Triggers>
    </wv2:WebView2>

*/

[Microsoft.Xaml.Behaviors.TypeConstraint(typeof(Microsoft.Web.WebView2.Wpf.WebView2))]
public class MessageAction : Microsoft.Xaml.Behaviors.TriggerAction<DependencyObject> {
  protected override void Invoke(object? parameter) {
    ConsoleEx.WriteNote("call Invoke");
    if(parameter is DependencyPropertyChangedEventArgs){ // Trigger = PropertyChanged
      ConsoleEx.WriteNote($$"""   oldval {{((DependencyPropertyChangedEventArgs)parameter).OldValue}}""");
      ConsoleEx.WriteNote($$"""   newval {{((DependencyPropertyChangedEventArgs)parameter).NewValue}}""");
      ((WebView2)this.AssociatedObject)?.CoreWebView2.ExecuteScriptAsync($$"""console.log("Invoke")""");
    }
    if(parameter is RoutedEventArgs){
      ConsoleEx.WriteNote(((RoutedEventArgs)parameter).RoutedEvent.Name); // Trigger = Loaded
    }
  }
}



[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class HostObject {
  public WebView2? wv;

  struct EventMessager {
    public string type { get; set; }
    public object? data { get; set; }

    public EventMessager(string type, object? data){ this.type = type; this.data = data; }
    public string to_json() => System.Text.Json.JsonSerializer.Serialize(this);
  };

  public async Task<bool> ExecuteScriptAsync(string path) {
      var src = await ReadAllString(path);
      await wv!.CoreWebView2.ExecuteScriptAsync(src);
      return true; 
  }

  public async Task<string> ReadAllString(string path) {
      var assemblyName = System.Reflection.Assembly.GetExecutingAssembly().GetName().Name;
      var isResource = path.StartsWith($"{assemblyName}.");
      var isUrl = Uri.IsWellFormedUriString(path, UriKind.Absolute);

      async Task<string> GetUrl(string path){
          using var client = new System.Net.Http.HttpClient();
          var response = await client.GetAsync(path);
          return await response.Content.ReadAsStringAsync();
      }
      async Task<string> GetAssembly(string path){
          var assembly = System.Reflection.Assembly.GetExecutingAssembly();
          using var stream = assembly.GetManifestResourceStream(path);
          if(stream is null) return ""; 
          using var streamReader = new System.IO.StreamReader(stream);
          return await streamReader.ReadToEndAsync();
      }
      return (isUrl, isResource) switch {
        /* web      */ (true, _) => await GetUrl(path),
        /* resource */ (false, true) => await GetAssembly(path),
        /* local    */ (_, _) => await File.ReadAllTextAsync(System.IO.Path.Combine(System.IO.Directory.GetCurrentDirectory(), path))
      };
  }

  public void OpenDevToolsWindow() => this.wv!.CoreWebView2.OpenDevToolsWindow();

  public double ZoomFactor { get => this.wv!.ZoomFactor; set{ this.wv!.ZoomFactor = value; } }

}



[ContentProperty("Content")]
public class WebView2Behavior : Microsoft.Xaml.Behaviors.Behavior<WebView2> {

  HostObject hostObject = new HostObject();

  /*** Content (ContentProperty) ***/

  #region Content

  public static readonly DependencyProperty ContentProperty = DependencyProperty.Register(
    nameof(Content),
    typeof(string),
    typeof(WebView2Behavior),
    new UIPropertyMetadata(null, ContentPropertyChanged)
  );
  public string Content { get => (string)GetValue(ContentProperty); set => SetValue(ContentProperty, value); }
  
  private static void ContentPropertyChanged(DependencyObject d, DependencyPropertyChangedEventArgs e) {
    ConsoleEx.WriteInfo("");
    var wb = d as WebView2Behavior;
    var target = e.NewValue as string;
    if (wb?.hostObject.wv?.CoreWebView2 is null || target is null) return;
    WebViewEx.NavigateRaw(wb.hostObject.wv, target);
  }

  #endregion

  /*** RegistScript ***/

  #region RegistScript

  private Dictionary<string, string> RegistJavaSciptList = new Dictionary<string, string>();

  public static readonly DependencyProperty RegistScriptProperty = DependencyProperty.Register(
    nameof(RegistScript),
    typeof(string),
    typeof(WebView2Behavior),
    new UIPropertyMetadata(null, RegistScriptPropertyChanged)
  );
  public string RegistScript { get => (string)GetValue(RegistScriptProperty); set => SetValue(RegistScriptProperty, value); }

  private static async void RegistScriptPropertyChanged(DependencyObject d, DependencyPropertyChangedEventArgs e) {
    ConsoleEx.WriteInfo("");
    var wb = d as WebView2Behavior;
    var target = e.NewValue as string;
    if (wb?.hostObject.wv?.CoreWebView2 is null || target is null) return;
    WebViewEx.RemoveJavaScript(wb.AssociatedObject, wb.RegistJavaSciptList, e.OldValue as string ?? "[]");
    await WebViewEx.RegisteJavaScript(wb.AssociatedObject, wb.RegistJavaSciptList, target);
  }

  #endregion
  
  /*** Action ***/
  
  #region ExecuteScript
  
  public static readonly DependencyProperty ExecuteScriptProperty = DependencyProperty.Register(
    nameof(ExecuteScript),
    typeof(Func<string, Task<string>>),
    typeof(WebView2Behavior),
    new PropertyMetadata(null)
  );
  public Func<string,Task<string>> ExecuteScript { get => (Func<string,Task<string>>)GetValue(ExecuteScriptProperty); set => SetValue(ExecuteScriptProperty, value); }

  public void AddActionExecuteScript(WebView2 wv) {
    if(wv is null) throw new Exception();
    this.ExecuteScript = new Func<string, Task<string>>(async (content) => {
      ConsoleEx.WriteInfo($$"""execute {{content}}""");
      return await WebViewEx.ExecuteScript(wv, content);
    });
  }

  #endregion

  /*** ICommand ***/
  
  #region ICommand

  public static readonly DependencyProperty BehaviorLoadedProperty = DependencyProperty.Register(
    nameof(BehaviorLoaded), 
    typeof(ICommand), 
    typeof(WebView2Behavior), 
    new PropertyMetadata(null)
  );
  public ICommand BehaviorLoaded { get => (ICommand)GetValue(BehaviorLoadedProperty); set => SetValue(BehaviorLoadedProperty, value);}

  public static readonly DependencyProperty DispatchedCommandProperty = DependencyProperty.Register(
    nameof(DispatchedCommand), 
    typeof(ICommand), 
    typeof(WebView2Behavior), 
    new PropertyMetadata(null)
  );
  public ICommand DispatchedCommand { get => (ICommand)GetValue(DispatchedCommandProperty); set => SetValue(DispatchedCommandProperty, value);}

  #endregion

  /*** Property ***/

  #region Property

  public static DependencyProperty ModelTypeProperty = DependencyProperty.Register(
    nameof(ModelType),
    typeof(Type),
    typeof(WebView2Behavior),
    new PropertyMetadata(null)
  );
  public Type ModelType { get => (Type)GetValue(ModelTypeProperty); set => SetValue(ModelTypeProperty, value);}


  public static DependencyProperty NavigateProperty = DependencyProperty.Register(
    nameof(Navigate),
    typeof(string),
    typeof(WebView2Behavior),
    new UIPropertyMetadata(null, NavigatePropertyChanged)
  );
  public string Navigate { get => (string)GetValue(NavigateProperty); set => SetValue(NavigateProperty, value);}

  private static void NavigatePropertyChanged(DependencyObject d, DependencyPropertyChangedEventArgs e) {
    var wb = d as WebView2Behavior;
    var target = e.NewValue as string;
    if (wb?.hostObject.wv?.CoreWebView2 is null) return;
    if (target is null) return;
    ConsoleEx.WriteInfo($$"""{{target}}""");
    WebViewEx.Navigate(wb.hostObject.wv, target);
  }

  #endregion

  #region OnAttached and Initialization

  protected override void OnAttached() {
    ConsoleEx.WriteInfo("start OnAttached");
    base.OnAttached();

    /*** initialize after webview init / parallel operation with CoreWebView2InitializationCompleted ***/
    Observable.FromEventPattern<RoutedEventHandler,RoutedEventArgs>(
      handler => { AssociatedObject.Loaded += handler; ConsoleEx.WriteInfo("add AssociatedObject.Loaded"); },
      handler => { AssociatedObject.Loaded -= handler; ConsoleEx.WriteInfo("remove AssociatedObject.Loaded"); }
    ).Subscribe( async _=>{
      ConsoleEx.WriteInfo("AssociatedObject.Loaded");
      ConsoleEx.WriteInfo("initializing webview");
      this.hostObject.wv = await WebViewEx.InitWebView(AssociatedObject);

      /*add binding action*/

      AddActionExecuteScript(this.hostObject.wv);

      /*add HostObject*/
      this.hostObject.wv.CoreWebView2.AddHostObjectToScript("Window", this.hostObject);
      ConsoleEx.WriteInfo("regist HostObjectToScript Window");
      WebViewEx.AddModel(this.hostObject.wv, ModelType);
      ConsoleEx.WriteInfo($"regist HostObjectToScript Model / regist PropertyChanged : {ModelType}");
      // Ioc.Default.GetRequiredService<NamedPipe>().PipeMessageReceived += (s, e) => {
      //     this.Dispatcher.Invoke(() => {
      //         var dst = System.Text.Json.JsonSerializer.Deserialize<object>(e);
      //         wv.CoreWebView2.PostWebMessageAsJson(new EventMessager("pipeMessageReceived", dst).to_json());
      //     });
      // };

      /*add javascript*/

      await WebViewEx.RegisteJavaScript(this.hostObject.wv, RegistJavaSciptList, RegistScript);

      /* CustomTag test */
      await WebViewEx.AddCustomTag(this.hostObject.wv, obj);
      
      WebViewEx.NavigateRaw(this.hostObject.wv, Content);
      BehaviorLoaded.Execute(null);
    });

    Observable.FromEventPattern<CoreWebView2InitializationCompletedEventArgs>(
      handler => { AssociatedObject.CoreWebView2InitializationCompleted += handler; ConsoleEx.WriteInfo("add CoreWebView2InitializationCompleted"); },
      handler => { AssociatedObject.CoreWebView2InitializationCompleted -= handler; ConsoleEx.WriteInfo("remove CoreWebView2InitializationCompleted"); }
    ).Subscribe(_=>{
      /* 
        postMessage('example')  -TryGetWebMessageAsString()-> "example"
        postMessage({'a': 'b'}) -TryGetWebMessageAsString()-> ArgumentException
        postMessage(1.2)        -TryGetWebMessageAsString()-> ArgumentException

        postMessage('example')  -WebMessageAsJson-> "\"example\""
        postMessage({'a': 'b'}) -WebMessageAsJson-> "{\"a\": \"b\"}"
        postMessage(1.2)        -WebMessageAsJson-> "1.2"
        
        <i:Interaction.Triggers>
          <i:EventTrigger EventName="WebMessageReceived">
              <i:InvokeCommandAction Command="{Binding WebMessageReceived}" PassEventArgsToCommand="True"/>
          </i:EventTrigger>
        </i:Interaction.Triggers>
        で伝播する
      */
      Observable.FromEventPattern<CoreWebView2WebMessageReceivedEventArgs>(
        handler => { AssociatedObject.CoreWebView2.WebMessageReceived += handler; ConsoleEx.WriteInfo("add CoreWebView2.WebMessageReceived"); },
        handler => { AssociatedObject.CoreWebView2.WebMessageReceived -= handler; ConsoleEx.WriteInfo("remove CoreWebView2.WebMessageReceived"); }
      ).Subscribe( e => DispatchedCommand!.Execute(e.EventArgs?.WebMessageAsJson ?? "{}") );
      /* CoreWebView2.WebMessageReceived はxmlで伝播しないのでBehaviorで */
      Observable.FromEventPattern<CoreWebView2NewWindowRequestedEventArgs>(
        handler => { AssociatedObject.CoreWebView2.NewWindowRequested += handler; ConsoleEx.WriteInfo("add CoreWebView2.NewWindowRequested"); },
        handler => { AssociatedObject.CoreWebView2.NewWindowRequested -= handler; ConsoleEx.WriteInfo("remove CoreWebView2.NewWindowRequested"); }
      ).Subscribe( e => {
        this.hostObject.wv?.CoreWebView2.PostWebMessageAsJson(new WebViewEx.EventMessager("newWindowRequested", e.EventArgs).to_json());
        e.EventArgs.Handled = true; 
      });
    });
  
  }

  #endregion
  CustomTag obj = new CustomTag();
}

public static class WebViewEx {

  public struct EventMessager {
    public string type { get; set; }
    public object? data { get; set; }

    public EventMessager(string type, object? data){ this.type = type; this.data = data; }
    public string to_json() => System.Text.Json.JsonSerializer.Serialize(this);
  };

  public static async Task<WebView2> InitWebView(WebView2 wv) {
    var webview_options = new CoreWebView2EnvironmentOptions("--allow-file-access-from-files");
    var environment = await CoreWebView2Environment.CreateAsync(null, null, webview_options);
    await wv.EnsureCoreWebView2Async(environment); // WebView2初期化完了確認
    return wv;
  }

  public static void AddModel(WebView2 wv, Type serviceType) {
    var model = Ioc.Default.GetService(serviceType);
    wv.CoreWebView2.AddHostObjectToScript("Model", model);
    Observable.FromEventPattern<PropertyChangedEventHandler, PropertyChangedEventArgs>(
      handler => { (model as INotifyPropertyChanged)!.PropertyChanged += handler; ConsoleEx.WriteInfo("add Model.PropertyChanged"); },
      handler => { (model as INotifyPropertyChanged)!.PropertyChanged -= handler; ConsoleEx.WriteInfo("remove Model.PropertyChanged"); }
    ).Subscribe(e=>{ wv.CoreWebView2.PostWebMessageAsJson(new EventMessager("propertyChanged", e.EventArgs).to_json()); });
  }

  public static async Task AddCustomTag(WebView2 wv, CustomTag obj) {
    var key = "CustomTag";
    wv.CoreWebView2.AddHostObjectToScript(key, obj);
    var id = await wv.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync(obj.JavaScript);
    Observable.FromEventPattern<PropertyChangedEventHandler, PropertyChangedEventArgs>(
      handler => { (obj as INotifyPropertyChanged)!.PropertyChanged += handler; ConsoleEx.WriteInfo($"add {key}.PropertyChanged"); },
      handler => { (obj as INotifyPropertyChanged)!.PropertyChanged -= handler; ConsoleEx.WriteInfo($"remove {key}.PropertyChanged"); }
    ).Subscribe(e=>{ wv.CoreWebView2.PostWebMessageAsJson(new EventMessager($"{key}PropertyChanged", e.EventArgs).to_json()); });

    // wv.CoreWebView2.RemoveHostObjectFromScript(key);
    // wv.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(id);
  }


  public static string res_to_contents(this string path) {
    var assembly = System.Reflection.Assembly.GetExecutingAssembly();
    using var stream = assembly.GetManifestResourceStream(path);
    if(stream is null) return ""; 
    using var streamReader = new System.IO.StreamReader(stream);
    var code = streamReader.ReadToEnd();
    return code;
  }

  public static List<string> reg_to_resPaths(this string reg) {
    var assembly = System.Reflection.Assembly.GetExecutingAssembly();
    var paths = assembly.GetManifestResourceNames().ToList().FindAll(n => System.Text.RegularExpressions.Regex.IsMatch(n, reg));
    return paths;
  }

  public static Task<string> ExecuteScript(WebView2 webView, string src) {
    if (webView != null && webView.CoreWebView2 != null) {
      var assemblyName = System.Reflection.Assembly.GetExecutingAssembly().GetName().Name;
      return (Uri.IsWellFormedUriString(src, UriKind.Absolute), src.StartsWith($"{assemblyName}.")) switch {
        /* resource */ (_, true) => webView.CoreWebView2.ExecuteScriptAsync(src.res_to_contents()),
        /* web      */ (true, false) => throw new Exception(),
        /* local    */ (false, false) => throw new Exception(),
      };
    }else{
      throw new Exception();
    }
  }

  public static void NavigateRaw(WebView2 webView, string src) {
    if (webView?.CoreWebView2 is not null) {
      webView.CoreWebView2.NavigateToString(src);
    }
  }
  public static void Navigate(WebView2 webView, string src) {
    if (webView != null && webView.CoreWebView2 != null) {
      var assemblyName = System.Reflection.Assembly.GetExecutingAssembly().GetName().Name;
      ((Uri.IsWellFormedUriString(src, UriKind.Absolute), src.StartsWith($"{assemblyName}.")) switch {
        /* resource */ (_, true) => (Action)(() => webView.CoreWebView2.NavigateToString(src.res_to_contents())),
        /* web      */ (true, false) => (Action)(()=> webView.CoreWebView2.Navigate(src)),
        /* local    */ (false, false) => (Action)(() => webView.CoreWebView2.Navigate(System.IO.Path.Combine(System.IO.Directory.GetCurrentDirectory(), src)))
      })();
    }
  }


  public static async Task<bool> RegisteJavaScript(WebView2 webView, Dictionary<string,string> list, string json, bool immediately_invok = false) {
    var paths = System.Text.Json.JsonSerializer.Deserialize<List<string>>(json) ?? new List<string>();
    foreach(var path in paths) {
      if(list.ContainsKey(path)) { ConsoleEx.WriteInfo($$"""skip regist {{path}}"""); continue; }
      var id = await webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync(path.res_to_contents());
      list.Add(path, id);
      ConsoleEx.WriteInfo($$"""regist {{path}} : {{id}}""");

      if(immediately_invok) {
        await webView.ExecuteScriptAsync(path.res_to_contents());
      }
    }
    return true;
  }

  public static void RemoveJavaScript(WebView2 webView, Dictionary<string,string> list, string oldJson) {
    var paths = System.Text.Json.JsonSerializer.Deserialize<List<string>>(oldJson) ?? new List<string>();
    foreach(var path in paths) {
      if(!list.ContainsKey(path)) { ConsoleEx.WriteInfo($$"""remove regist {{path}} : no id"""); continue; }
      webView.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(list[path]);
      ConsoleEx.WriteInfo($$"""remove regist {{path}} : {{list[path]}}""");
      list.Remove(path);
    }
  }


  // public async Task<string> AddScript(string key, string code, bool immediately_invok = false) {
  //   if(id.ContainsKey(key)) { 
  //     webView.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(id[key]);
  //     id.Remove(key);
  //   }
  //   var result = await webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync(code);
  //   if(immediately_invok) {
  //     await webView.ExecuteScriptAsync(code);
  //   }
  //   id.Add(key, result);
  //   return result;
  // }
  // public void RemoveScript(string key) {
  //   if(id.ContainsKey(key)){
  //     webView.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(id[key]);
  //     id.Remove(key);
  //   }
  // }

  public static void ShowMessageBoxLite(this string s) => System.Windows.MessageBox.Show(s);

}
