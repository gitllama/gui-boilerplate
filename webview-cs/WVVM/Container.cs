using System;
using Microsoft.Extensions.DependencyInjection;

namespace WVVM;

/*
  var services = new ServiceCollection();
  services.AddSingleton<ISomeClient, SomeClient>();

  var provider = services.BuildServiceProvider();
  var someClient1 = provider.GetRequiredService<ISomeClient>();
*/
public sealed class Ioc : IServiceProvider {
  public static Ioc Default { get; } = new Ioc();
  private volatile ServiceProvider? serviceProvider;

  /* GetService */

  object? IServiceProvider.GetService(Type serviceType) {
    ServiceProvider? provider = this.serviceProvider;
    if (provider is null) { ThrowInvalidOperationExceptionForMissingInitialization(); }
    return provider!.GetService(serviceType);
  }

  public object? GetService(Type serviceType) {
    if(this.serviceProvider is null) throw new Exception();
    return this.serviceProvider.GetService(serviceType);
  }

  public T? GetRequiredKeyedService<T>(string key) where T : notnull {
    if(this.serviceProvider is null) throw new Exception();
    return this.serviceProvider.GetRequiredKeyedService<T>(key);
  }

  /* ConfigureServices */

  public void ConfigureServices(Action<IServiceCollection> setup) => ConfigureServices(setup, new ServiceProviderOptions());

  public void ConfigureServices(Action<IServiceCollection> setup, ServiceProviderOptions options) {
    var collection = new ServiceCollection();
    setup(collection);
    ConfigureServices(collection, options);
  }

  public void ConfigureServices(IServiceCollection services) => ConfigureServices(services, new ServiceProviderOptions());

  public void ConfigureServices(IServiceCollection services, ServiceProviderOptions options) {
    ServiceProvider newServices = services.BuildServiceProvider(options);
    ServiceProvider? oldServices = Interlocked.CompareExchange(ref this.serviceProvider, newServices, null);
    if (!(oldServices is null)) { ThrowInvalidOperationExceptionForRepeatedConfiguration(); }
  }


  private static void ThrowInvalidOperationExceptionForMissingInitialization() => throw new InvalidOperationException("The service provider has not been configured yet");
  private static void ThrowInvalidOperationExceptionForRepeatedConfiguration() => throw new InvalidOperationException("The default service provider has already been configured");

}
