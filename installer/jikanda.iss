; Copyright (C) 2026 Yu Prananda
; SPDX-License-Identifier: GPL-3.0-only
; Build with the unmodified official Inno Setup 7.1.1 compiler.
#ifndef AppVersion
  #define AppVersion "0.1.0-beta.1"
#endif
#ifndef PayloadDir
  #error Define PayloadDir to the verified extracted Windows release ZIP
#endif
#ifndef ReleaseDir
  #error Define ReleaseDir for the output installer
#endif
#ifndef IconFile
  #error Define IconFile to the approved Jikanda ICO
#endif

[Setup]
AppId=YuPrananda.Jikanda
AppName=Jikanda
AppVersion={#AppVersion}
AppVerName=Jikanda {#AppVersion}
AppPublisher=Yu Prananda
AppPublisherURL=https://github.com/yuprananda/jikanda
AppSupportURL=https://github.com/yuprananda/jikanda/issues
AppUpdatesURL=https://github.com/yuprananda/jikanda/releases
VersionInfoVersion=0.1.0.1
VersionInfoProductVersion=0.1.0.1
VersionInfoProductTextVersion={#AppVersion}
VersionInfoTextVersion={#AppVersion}
VersionInfoCopyright=Copyright (C) 2026 Yu Prananda. GPL-3.0-only.
DefaultDirName={localappdata}\Programs\Jikanda
DefaultGroupName=Jikanda
PrivilegesRequired=lowest
SetupArchitecture=x64
ArchitecturesAllowed=x64os
ArchitecturesInstallIn64BitMode=x64os
MinVersion=10.0.22000
WizardStyle=modern
DisableProgramGroupPage=yes
LicenseFile={#PayloadDir}\LICENSE
SetupIconFile={#IconFile}
UninstallDisplayIcon={app}\jikanda.exe
OutputDir={#ReleaseDir}
OutputBaseFilename=Jikanda-{#AppVersion}-windows-x64-setup
Compression=lzma2
SolidCompression=yes
CloseApplications=no
RestartApplications=no
Uninstallable=yes
UninstallDisplayName=Jikanda
AllowNoIcons=yes
UsePreviousAppDir=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Shortcuts:"; Flags: unchecked

[Files]
Source: "{#PayloadDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Jikanda"; Filename: "{app}\jikanda.exe"; WorkingDir: "{app}"; IconFilename: "{app}\jikanda.exe"; AppUserModelID: "YuPrananda.Jikanda"
Name: "{autodesktop}\Jikanda"; Filename: "{app}\jikanda.exe"; WorkingDir: "{app}"; IconFilename: "{app}\jikanda.exe"; AppUserModelID: "YuPrananda.Jikanda"; Tasks: desktopicon

[Run]
Filename: "{app}\jikanda.exe"; Description: "Open Jikanda"; Flags: nowait postinstall skipifsilent unchecked

[Code]
function InitializeSetup: Boolean;
begin
  Result := FileExists(ExpandConstant('{sys}\vcruntime140.dll'));
  if not Result then
    SuppressibleMsgBox('Jikanda requires the Microsoft Visual C++ x64 runtime.' + #13#10 +
      'Install it from https://aka.ms/vs/17/release/vc_redist.x64.exe and run Setup again.', mbError, MB_OK, IDOK);
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ExistingExe: String;
  ExistingFile: TFileStream;
begin
  Result := '';
  ExistingExe := ExpandConstant('{app}\jikanda.exe');
  if FileExists(ExistingExe) then begin
    try
      ExistingFile := TFileStream.Create(ExistingExe, fmOpenReadWrite or fmShareExclusive);
      ExistingFile.Free;
    except
      Result := 'Close Jikanda before updating, then click Retry. Setup will not force-close an active session.';
    end;
  end;
end;

// No UninstallDelete rules: remove installed files/shortcuts only.
// User history, exports, preferences and unrelated files are preserved.
