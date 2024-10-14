using System.Windows;
using System.Windows.Controls;
using System.Runtime.InteropServices;
using WVVM;
using Microsoft.Extensions.DependencyInjection;

namespace WVVMSample;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window {
    public MainWindow(ServiceCollection collection) {
        ConsoleEx.WriteInfo("InitializeComponent");
        /* this.DataContext = new MainViewModel(); */
        InitializeComponent();
    }
}

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class TextBlockJs : TextBlock { }
