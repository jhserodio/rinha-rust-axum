NETSDK1080: A PackageReference to Microsoft.AspNetCore.App is not necessary when targeting .NET Core 3.0 or higher. If Microsoft.NET.Sdk.Web is used, the shared framework will be referenced automatically. Otherwise, the PackageReference should be replaced with a FrameworkReference. [Tecmundo.WebCore\Tecmundo.WebCore.csproj]
NU1803: You are running the 'restore' operation with an 'HTTP' source, 'http://nuget.nznweb.com.br/nuget'. Non-HTTPS access will be removed in a future version. Consider migrating to an 'HTTPS' source. [Tecmundo.sln]
NU1902: Package 'System.Data.SqlClient' 4.6.0 has a known moderate severity vulnerability, https://github.com/advisories/GHSA-8g2p-5pqh-5jmc [Tecmundo.sln]
NU1903: Package 'RestSharp' 106.11.5 has a known high severity vulnerability, https://github.com/advisories/GHSA-9pq7-rcxv-47vq [Tecmundo.sln]
NU1701: Package 'CsQuery 1.3.4' was restored using '.NETFramework,Version=v4.6.1, .NETFramework,Version=v4.6.2, ...' instead of the project target framework 'net6.0'. This package may not be fully compatible with your project. [Tecmundo.sln]
NU1701: Package 'NZN.Util 4.0.3' was restored using '.NETFramework,Version=v4.6.1, .NETFramework,Version=v4.6.2, ...' instead of the project target framework 'net6.0'. This package may not be fully compatible with your project. [Tecmundo.sln]
NU1902: Package 'HtmlSanitizer' 4.0.217 has a known moderate severity vulnerability, https://github.com/advisories/GHSA-43cp-6p3q-2pc4 [Tecmundo.sln]
NU1901: Package 'HtmlSanitizer' 4.0.217 has a known low severity vulnerability, https://github.com/advisories/GHSA-8j9v-h2vp-2hhv [Tecmundo.sln]
CS0618: 'PlaylistItemSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Domain\Tecmundo.Domain.csproj]
CS0168: The variable 'e' is declared but never used [Tecmundo.Domain\Tecmundo.Domain.csproj]
SYSLIB0014: 'WebRequest.Create(string)' is obsolete: 'WebRequest, HttpWebRequest, ServicePoint, and WebClient are obsolete. Use HttpClient instead.' (https://aka.ms/dotnet-warnings/SYSLIB0014) [Tecmundo.Domain\Tecmundo.Domain.csproj]
Tecmundo.Domain -> Tecmundo.Domain\bin\Debug\net6.0\Tecmundo.Domain.dll
CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.Application\Tecmundo.Application.csproj]
CS0108: 'DetalhesProdutoViewModel.UrlLoja' hides inherited member 'ProdutoBaseViewModel.UrlLoja'. Use the new keyword if hiding was intended. [Tecmundo.Application\Tecmundo.Application.csproj]
CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.Application\Tecmundo.Application.csproj]
CS0618: 'VideoSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Application\Tecmundo.Application.csproj]
CS0618: 'PlaylistSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Application\Tecmundo.Application.csproj]
CS0168: The variable 'e' is declared but never used [Tecmundo.Application\Tecmundo.Application.csproj]
Tecmundo.Domain -> Tecmundo.Domain\bin\Debug\net6.0\Tecmundo.Domain.dll
CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.Infrastructure\Tecmundo.Infrastructure.csproj]
CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.Infrastructure\Tecmundo.Infrastructure.csproj]
CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.WebAPI\Tecmundo.WebAPI.csproj]
CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.WebAPI\Tecmundo.WebAPI.csproj]
The build stopped unexpectedly because of an internal failure.
Tecmundo.WebAPI -> Tecmundo.WebAPI\bin\Debug\net6.0\Tecmundo.WebAPI.dll
warning CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]
warning CS0168: The variable 'leaderboardSlotPath' is declared but never used [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]
warning UA0001: Namespace 'System.Web' should not be referenced in ASP.NET Core projects [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]


GPT irei te dar uma lista, me responda em portugues Brazil.

Essa lista é uma lista de erros em C#, dot net 6, de um CMS em padrão MVC com server side rendering, rodando em uma plataforma Kubernetes.

Quero que me de os principais problemas  que podem ocorrer de performance e infraestrutura por conta de cada um desses erros.



    NETSDK1080: A PackageReference to Microsoft.AspNetCore.App is not necessary when targeting .NET Core 3.0 or higher. If Microsoft.NET.Sdk.Web is used, the shared framework will be referenced automatically. Otherwise, the PackageReference should be replaced with a FrameworkReference. [Tecmundo.WebCore\Tecmundo.WebCore.csproj]
        Possíveis Problemas:
            Desnecessária referência à Microsoft.AspNetCore.App pode aumentar o tamanho da aplicação.
            Impacto na inicialização e carga devido ao carregamento de recursos desnecessários.

    NU1803: You are running the 'restore' operation with an 'HTTP' source, 'http://nuget.nznweb.com.br/nuget'. Non-HTTPS access will be removed in a future version. Consider migrating to an 'HTTPS' source. [Tecmundo.sln]
        Possíveis Problemas:
            Vulnerabilidade de segurança ao usar uma fonte de pacotes HTTP em vez de HTTPS.
            Risco de não conseguir restaurar pacotes em versões futuras do NuGet.

    NU1902: Package 'System.Data.SqlClient' 4.6.0 has a known moderate severity vulnerability, https://github.com/advisories/GHSA-8g2p-5pqh-5jmc [Tecmundo.sln]
        Possíveis Problemas:
            Vulnerabilidades de segurança podem ser exploradas, comprometendo a integridade dos dados.
            Possíveis falhas devido a correções de segurança ausentes.

    NU1903: Package 'RestSharp' 106.11.5 has a known high severity vulnerability, https://github.com/advisories/GHSA-9pq7-rcxv-47vq [Tecmundo.sln]
        Possíveis Problemas:
            Alta vulnerabilidade pode resultar em exploração maliciosa.
            Falhas de segurança que podem comprometer a comunicação da aplicação.

    NU1701: Package 'CsQuery 1.3.4' was restored using '.NETFramework,Version=v4.6.1, .NETFramework,Version=v4.6.2, ...' instead of the project target framework 'net6.0'. This package may not be fully compatible with your project. [Tecmundo.sln]
        Possíveis Problemas:
            Incompatibilidade entre a versão do pacote CsQuery e o projeto .NET 6.
            Funcionalidades mais recentes podem não estar disponíveis devido à incompatibilidade.

    NU1701: Package 'NZN.Util 4.0.3' was restored using '.NETFramework,Version=v4.6.1, .NETFramework,Version=v4.6.2, ...' instead of the project target framework 'net6.0'. This package may not be fully compatible with your project. [Tecmundo.sln]
        Possíveis Problemas:
            Incompatibilidade entre a versão do pacote NZN.Util e o projeto .NET 6.
            Funcionalidades mais recentes podem não estar disponíveis devido à incompatibilidade.

    NU1902: Package 'HtmlSanitizer' 4.0.217 has a known moderate severity vulnerability, https://github.com/advisories/GHSA-43cp-6p3q-2pc4 [Tecmundo.sln]
        Possíveis Problemas:
            Vulnerabilidades no sanitizador HTML podem permitir ataques de injeção ou XSS.
            Possibilidade de comprometimento da segurança da aplicação.

    NU1901: Package 'HtmlSanitizer' 4.0.217 has a known low severity vulnerability, https://github.com/advisories/GHSA-8j9v-h2vp-2hhv [Tecmundo.sln]
        Possíveis Problemas:
            Mesmo sendo de baixa severidade, vulnerabilidades podem ser exploradas.
            Ações maliciosas podem ocorrer devido à exploração dessas vulnerabilidades.

    CS0618: 'PlaylistItemSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Domain\Tecmundo.Domain.csproj]
        Possíveis Problemas:
            Uso de propriedades obsoletas pode resultar em comportamento inesperado.
            Impacto nas operações relacionadas a PlaylistItemSnippet.

    CS0168: The variable 'e' is declared but never used [Tecmundo.Domain\Tecmundo.Domain.csproj]

    Possíveis Problemas:
        Alocação desnecessária de recursos devido à variável não utilizada.
        Potencial impacto na eficiência do código.

    SYSLIB0014: 'WebRequest.Create(string)' is obsolete: 'WebRequest, HttpWebRequest, ServicePoint, and WebClient are obsolete. Use HttpClient instead.' (https://aka.ms/dotnet-warnings/SYSLIB0014) [Tecmundo.Domain\Tecmundo.Domain.csproj]

    Possíveis Problemas:
        Obsolescência de métodos de requisição pode levar a falhas de interoperabilidade.
        Impacto na comunicação e possível falta de suporte em versões futuras.

    CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Possíveis conflitos de namespace devido à diretiva 'using' duplicada.
        Impacto na leitura e manutenção do código.

    CS0108: 'DetalhesProdutoViewModel.UrlLoja' hides inherited member 'ProdutoBaseViewModel.UrlLoja'. Use the new keyword if hiding was intended. [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Ocultação de membros herdados pode levar a confusões na leitura do código.
        Potencial impacto nas funcionalidades relacionadas a DetalhesProdutoViewModel.

    CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Métodos assíncronos executados de forma síncrona podem bloquear a thread principal.
        Possível degradação no desempenho, especialmente em operações de I/O.

    CS0618: 'VideoSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Uso de propriedades obsoletas em VideoSnippet pode levar a resultados inesperados.
        Impacto nas operações relacionadas a VideoSnippet.

    CS0618: 'PlaylistSnippet.PublishedAt' is obsolete: 'This property is obsolete and may behave unexpectedly; please use PublishedAtDateTimeOffset instead.' [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Uso de propriedades obsoletas em PlaylistSnippet pode levar a resultados inesperados.
        Impacto nas operações relacionadas a PlaylistSnippet.

    CS0168: The variable 'e' is declared but never used [Tecmundo.Application\Tecmundo.Application.csproj]

    Possíveis Problemas:
        Alocação desnecessária de recursos devido à variável não utilizada.
        Potencial impacto na eficiência do código.

    CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.Infrastructure\Tecmundo.Infrastructure.csproj]

    Possíveis Problemas:
        Possíveis conflitos de namespace devido à diretiva 'using' duplicada.
        Impacto na leitura e manutenção do código.

    CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.Infrastructure\Tecmundo.Infrastructure.csproj]

    Possíveis Problemas:
        Métodos assíncronos executados de forma síncrona podem bloquear a thread principal.
        Possível degradação no desempenho, especialmente em operações de I/O.

    CS0105: The using directive for 'System.Linq' appeared previously in this namespace [Tecmundo.WebAPI\Tecmundo.WebAPI.csproj]

    Possíveis Problemas:
        Possíveis conflitos de namespace devido à diretiva 'using' duplicada.
        Impacto na leitura e manutenção do código.

    CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [Tecmundo.WebAPI\Tecmundo.WebAPI.csproj]

    Possíveis Problemas:
        Métodos assíncronos executados de forma síncrona podem bloquear a thread principal.
        Possível degradação no desempenho, especialmente em operações de I/O.

    The build stopped unexpectedly because of an internal failure. Tecmundo.WebAPI -> Tecmundo.WebAPI\bin\Debug\net6.0\Tecmundo.WebAPI.dll

    Possíveis Problemas:
        Parada inesperada pode resultar em interrupções na aplicação.
        Necessidade de investigar a causa da falha interna.

    warning CS1998: This async method lacks 'await' operators and will run synchronously. Consider using the 'await' operator to await non-blocking API calls, or 'await Task.Run(...)' to do CPU-bound work on a background thread. [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]

    Possíveis Problemas:
        Advertência sobre método assíncrono executado de forma síncrona pode indicar possíveis problemas de desempenho.
        Pode levar a bloqueios na thread principal, afetando a responsividade.

    warning CS0168: The variable 'leaderboardSlotPath' is declared but never used [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]

    Possíveis Problemas:
        Alocação desnecessária de recursos devido à variável não utilizada.
        Potencial impacto na eficiência do código.

    warning UA0001: Namespace 'System.Web' should not be referenced in ASP.NET Core projects [TecmundoMobile.WebCore\TecmundoMobile.WebCore.csproj]

    Possíveis Problemas:
        Referenciar o namespace 'System.Web' pode levar a incompatibilidades em projetos ASP.NET Core.
        Pode resultar em comportamento inesperado e falta de suporte em versões futuras.