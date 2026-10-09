#ifndef AppVersion
  #define AppVersion "0.1.1"
#endif

[Setup]
AppId={{68882DAE-3E09-4823-B1BB-D29D94D3BB30}
AppName=Kiniwolf Browser
AppVersion={#AppVersion}
AppPublisher=userpolopo-web
AppPublisherURL=https://github.com/userpolopo-web/kiniwolf
DefaultDirName={localappdata}\Programs\Kiniwolf
DefaultGroupName=Kiniwolf Browser
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir=..\..\dist
OutputBaseFilename=Kiniwolf-{#AppVersion}-windows-x64-setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\kiniwolf-browser.exe
CloseApplications=yes

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; Flags: unchecked

[Files]
Source: "..\..\target\release\kiniwolf-browser.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\MicrosoftEdgeWebview2Setup.exe"; Flags: dontcopy

[Icons]
Name: "{group}\Kiniwolf Browser"; Filename: "{app}\kiniwolf-browser.exe"
Name: "{autodesktop}\Kiniwolf Browser"; Filename: "{app}\kiniwolf-browser.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\kiniwolf-browser.exe"; Description: "Open Kiniwolf Browser"; Flags: nowait postinstall skipifsilent

[Code]
function HasWebView2: Boolean;
var
  Version: String;
  Key: String;
begin
  Key := 'SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}';
  Result := (RegQueryStringValue(HKCU, Key, 'pv', Version) and (Version <> '') and (Version <> '0.0.0.0')) or
    (RegQueryStringValue(HKLM32, Key, 'pv', Version) and (Version <> '') and (Version <> '0.0.0.0')) or
    (RegQueryStringValue(HKLM64, Key, 'pv', Version) and (Version <> '') and (Version <> '0.0.0.0'));
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ExitCode: Integer;
begin
  Result := '';
  if not HasWebView2 then
  begin
    ExtractTemporaryFile('MicrosoftEdgeWebview2Setup.exe');
    if not Exec(ExpandConstant('{tmp}\MicrosoftEdgeWebview2Setup.exe'), '/silent /install', '', SW_HIDE, ewWaitUntilTerminated, ExitCode) then
      Result := 'Could not start WebView2 installation. Install Microsoft Edge WebView2 Runtime and try again.'
    else if not HasWebView2 then
      Result := 'WebView2 installation did not complete. Check your internet connection and try again.';
  end;
end;
