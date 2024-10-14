using System;
using System.Runtime.InteropServices;
using System.ComponentModel;
using System.Windows.Input;
using System.Windows;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;

namespace WVVM;

// INotifyPropertyChanged
// IDataErrorInfo
// ICommand
public class ViewModelBase : INotifyPropertyChanged {

  public event PropertyChangedEventHandler? PropertyChanged = null;

  protected void OnPropertyChanged([System.Runtime.CompilerServices.CallerMemberName] string? propertyName = null) {
    PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
  }

}


public class ModelBase : INotifyPropertyChanged {

  public ModelBase() { }

  /*
    to code-behind
      webView.CoreWebView2.PostWebMessageAsString();
      or webView.CoreWebView2.PostWebMessageAsJson();
      or webView.ExecuteScriptAsync(" window.chrome.webview.dispatchEvent(new CustomEvent('propertyChanged', { ... });" });
  */
  public event PropertyChangedEventHandler? PropertyChanged;
  protected void OnPropertyChanged([System.Runtime.CompilerServices.CallerMemberName]string? propertyName = null) {
    PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
  }


  public object? this[string propertyName] {
    get => this.GetType().GetProperty(propertyName)?.GetValue(this) ?? null;
    set { this.GetType().GetProperty(propertyName)?.SetValue(this, value); }
  }

  public string[] GetPropertys() => this.GetType().GetProperties().Select(e => e.Name).ToArray();
  // string tp = n.PropertyType.ToString();

  protected struct Messenger {
    public string type { get; set; }
    public dynamic payload { get; set; }
    public T DeserializePayload<T>() => System.Text.Json.JsonSerializer.Deserialize<T>(payload.GetRawText());
  }
}


public class DelegateCommand : ICommand {
  Action<object?> execute = (n) => {};
  Func<bool> canExecute = () => false;
  
  public event EventHandler? CanExecuteChanged {
    add { System.Windows.Input.CommandManager.RequerySuggested += value; }
    remove { System.Windows.Input.CommandManager.RequerySuggested -= value; }
  }
  
  public bool CanExecute(object? parameter) { return canExecute(); }

  public void Execute(object? parameter) { execute(parameter); }
  
  public DelegateCommand(Action<object?> execute) => this.execute = execute;

  public DelegateCommand(Action<object?> execute, Func<bool> canExecute) {
    this.execute = execute;
    this.canExecute = canExecute;
  }

}
