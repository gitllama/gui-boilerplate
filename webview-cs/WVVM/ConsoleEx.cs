using System.Runtime.CompilerServices;

namespace WVVM;

public static class ConsoleEx {

  public enum AllocConsoleEnum {
    AttachConsole,
    AllocConsoleWindow,

    Default
  }

  private static volatile AllocConsoleEnum _setting = AllocConsoleEnum.Default;

  public static void Setting(AllocConsoleEnum val) {
    ConsoleEx._setting = val;
    switch(val) {
      case AllocConsoleEnum.AttachConsole:
        ConsoleEx.AttachConsole(-1);
        break;
      case AllocConsoleEnum.AllocConsoleWindow:
        ConsoleEx.AllocConsoleWindow();
        break;
    }
  }

  const string color_red    = "\u001b[31m";
  const string color_green  = "\u001b[32m";
  const string color_yellow = "\u001b[33m";
  const string color_blue   = "\u001b[34m";
  const string color_reset  = "\u001b[0m";

  public static string Red(this string src) => $"{color_red}{src}{color_reset}";
  
  public static string Green(this string src) => $"{color_green}{src}{color_reset}";
  
  public static string Yellow(this string src) => $"{color_yellow}{src}{color_reset}";
  
  public static string Blue(this string src) => $"{color_blue}{src}{color_reset}";


  // アタッチ先 : dotnet run -> 呼び出し元console, vscodeのF5 -> debug console
  [System.Runtime.InteropServices.DllImport("Kernel32.dll")]
  public static extern bool AttachConsole(int processId);

  [System.Runtime.InteropServices.DllImport("Kernel32.dll")]
  public static extern bool AllocConsole();

  public static void AllocConsoleWindow() {
    AllocConsole();
    var handle = GetStdHandle(0xFFFFFFF5); // StdOutputHandle = 0xFFFFFFF5
    GetConsoleMode(handle, out int oldMode);
    SetConsoleMode(handle, oldMode | 0x0004); // ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004
    // Console.SetOut(new System.IO.StreamWriter(Console.OpenStandardOutput()) { AutoFlush = true }); // 初回不要
  }
   
  [System.Runtime.InteropServices.DllImport("kernel32.dll")]
  static extern IntPtr GetStdHandle(UInt32 nStdHandle);

  [System.Runtime.InteropServices.DllImport( "kernel32.dll" )]
  static extern bool GetConsoleMode(IntPtr consoleHandle, out int mode);

  [System.Runtime.InteropServices.DllImport( "kernel32.dll" )]
  static extern bool SetConsoleMode(IntPtr consoleHandle, int mode);

  [System.Runtime.InteropServices.DllImport("Kernel32.dll")]
  public static extern bool FreeConsole();


  [MethodImpl(MethodImplOptions.NoInlining)]
  public static void WriteInfo(string? message, [CallerMemberName] string memberName = "", [CallerFilePath] string fileName = "") {
    var place = $"[{memberName} in {System.IO.Path.GetFileNameWithoutExtension(fileName)}]";
    Console.WriteLine($"{place.Green()} {message ?? ""}");
  }

  [MethodImpl(MethodImplOptions.NoInlining)]
  public static void WriteNote(string? message, [CallerMemberName] string memberName = "", [CallerFilePath] string fileName = "") {
    var place = $"[{memberName} in {System.IO.Path.GetFileNameWithoutExtension(fileName)}]";
    Console.WriteLine($"{place.Blue()} {message ?? ""}");
  }

  [MethodImpl(MethodImplOptions.NoInlining)]
  public static void WriteErr(string? message, [CallerMemberName] string memberName = "", [CallerFilePath] string fileName = "") {
    var place = $"[{memberName} in {System.IO.Path.GetFileNameWithoutExtension(fileName)}]";
    switch(_setting){
      case AllocConsoleEnum.AllocConsoleWindow:
        Console.WriteLine($"{message?.Red()}");
        Console.ReadKey();
        break;
      default:
        Console.WriteLine($"{message?.Red()}");
        System.Windows.MessageBox.Show(message);
        break;
    }
  }
}