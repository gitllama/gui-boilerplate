using CommandLine;
using System.Text.Json;

namespace WVVMSample;

public class Args {

  /******** CommandLine Parser ********/

  [Option('d', "devtool", Required = false, HelpText = "Open Chrome DevTools (Shift + Ctr + i)")]
  public bool devtool { get; set; }

  [Option('c', "cli", Required = false, HelpText = "Open Console")]
  public bool cli { get; set; }

  [Option('s', "starturl", Required = false, Default = @"Squid.resource.index.html", HelpText = "web url / local path / resource(Squid.resource.*)")]
  public string starturl { get; set; } = "";

  [Option('w', "working-directory", Required = false)]
  public string working { get; set; } = System.IO.Path.Combine(System.IO.Directory.GetCurrentDirectory(), "");

  [Option('o', "hostobjects-name", Required = false, Default = "Squid")]
  public string hostobjects { get; set; } = "";

  [Option('r', "no-register-javascript", Required = false)]
  public bool no_registjs { get; set; }

  // urlエンコードしてのurlパラメータ渡しはあまりキレイでないので
  [Option('a', "args", Required = false, Default = $$"""{"title" : "Squid"}""", HelpText = "inject ARGS into js")]
  public string args { get; set; } = "";


  public static string HelpText(ParserResult<Args>? parsed) {
    /*
      <-----Heading------------->
        <title> <version>
      <-----Copyright----------->
        Copyright (C) `<year> <Company>`
      <-----Error list---------->
      <-----Examples------------>
      <-----Pre_options--------->
      <-----Options_section----->
      <-----Post_options-------->
  */
  return CommandLine.Text.HelpText.AutoBuild(parsed, h => {
      // h.Heading = "aaa";
      // h.Heading = CommandLine.Text.HeadingInfo.Empty;
      h.AddDashesToOption = true;
      h.AdditionalNewLineAfterOption = false;
      h.AddNewLineBetweenHelpSections = true;
      h.AddEnumValuesToHelpText = true;
      h.AutoVersion = true;
      h.AutoHelp = true;
      return CommandLine.Text.HelpText.DefaultParsingErrorsHandler(parsed, h);
    }, e => e);
  }

  public static string VersionText(ParserResult<Args>? parsed) {
  return $"""
    {CommandLine.Text.HeadingInfo.Default}
    {CommandLine.Text.CopyrightInfo.Default}
    """;
  }

}

