using System.Windows;
using CommandLine;
using WVVM;
using Microsoft.Extensions.DependencyInjection;

namespace WVVMSample;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App : Application {
  private void Application_Startup(object sender, StartupEventArgs e) {
    Application.Current.DispatcherUnhandledException += OnDispatcherUnhandledException;

    /* Dependency Injection container */
    var collection = new ServiceCollection();

    using var parser = new Parser(with => with.HelpWriter = null);
    var parsed = parser.ParseArguments<Args>(Environment.GetCommandLineArgs());
    parsed.WithParsed<Args>(opt => {
      ConsoleEx.Setting(opt.cli ? ConsoleEx.AllocConsoleEnum.AllocConsoleWindow : ConsoleEx.AllocConsoleEnum.AttachConsole );
      ConsoleEx.WriteInfo("ConfigureServices");

      collection.AddSingleton<Args>(opt);
      collection.AddSingleton<MainModel>();
      collection.AddKeyedSingleton<ITest>("TestA", new TestA());
      collection.AddKeyedSingleton<ITest>("TestB", new TestB());

      Ioc.Default.ConfigureServices(collection);
      MainWindow wnd = new MainWindow(collection);

      ConsoleEx.WriteInfo("Window Show");
      wnd.Show();
    })
    .WithNotParsed(er => {
        ConsoleEx.Setting(ConsoleEx.AllocConsoleEnum.AttachConsole);
        if(er.IsHelp()) { Console.WriteLine(Args.HelpText(parsed)); }
        else if(er.IsVersion()) { Console.WriteLine(Args.VersionText(parsed)); }
        Environment.Exit(1);
    });
  }
  private void OnDispatcherUnhandledException(object sender, System.Windows.Threading.DispatcherUnhandledExceptionEventArgs e) {
      ConsoleEx.WriteErr($"OnDispatcherUnhandledException {e.Exception.ToString()}");
      Environment.Exit(1);
  }
}

public interface ITest {
  public string hoge { get; set; }
}

public class TestA : ITest{
  public string hoge { get; set; } = "TestA";
}

public class TestB : ITest{
  public string hoge { get; set; } = "TestB";
}
