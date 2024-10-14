using System;
using System.Runtime.InteropServices;
using System.ComponentModel;
using System.Windows.Threading;
using System.Windows;
using WVVM;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;

namespace WVVMSample;

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class MainModel : ModelBase {

  public MainModel() { ConsoleEx.WriteInfo("constructing"); }

  /* property */
  private string _ProcessMessage = "";
  public string ProcessMessage {
    get => _ProcessMessage;
    set {
      // if (value == _ProcessMessage) return;
      _ProcessMessage = value;
      OnPropertyChanged("ProcessMessage");
    }
  }

  private string _Label = "default";
  public string Label {
    get => _Label;
    set {
      if (value == _Label) return;
      _Label = value;
      OnPropertyChanged(); // RaisePropertyChanged(nameof(FullName));
    }
  }

  public string JavaScript => $$"""

  """;

  public void Reduce(string action) {
    var obj = System.Text.Json.JsonSerializer.Deserialize<Messenger>(action);
    switch(obj.type){
      case "message":
        this.ProcessMessage = obj.DeserializePayload<string>();
        return;
      case "error":
        this.ProcessMessage = obj.DeserializePayload<string>();
        return;
      default:
        ConsoleEx.WriteNote($"action type : {obj.type}");
        return;
    };
  }

}

public static class Dispatcher {

  struct Messenger {
    public string type { get; set; }
    public dynamic payload { get; set; }
    public T DeserializePayload<T>() => System.Text.Json.JsonSerializer.Deserialize<T>(payload.GetRawText());
  }

  /******** method ********/

  public static void dispatch(string json){
    try{
      // ConsoleEx.WriteNote(json);
      // var obj = System.Text.Json.JsonSerializer.Deserialize<Messenger>(json);
      // Action<Messenger>? act;
      // if (dispatcher.TryGetValue(obj.type, out act)) { act(obj); }
      // Reducer.GetInstance().Reduce(Ioc.Default.GetRequiredService<MainModel>(), json);
      var model = Ioc.Default.GetRequiredService<MainModel>();
      model.Reduce(json);
    }catch(Exception e){
      ConsoleEx.WriteNote(e.ToString());
    }
  }

}

