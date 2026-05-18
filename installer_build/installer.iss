; ============================================================
;  TechScript v1.0.7 — Inno Setup Installer Script
;  Professional Windows installer for the TechScript ecosystem.
;  Bundles: tech.exe (CLI), tech_studio.exe (IDE), VS Code ext,
;           examples, documentation, and icon assets.
; ============================================================

#define MyAppName "TechScript"
#define MyAppVersion "1.0.7"
#define MyAppPublisher "Tanmoy Majumder"
#define MyAppURL "https://github.com/Tcode-Motion/techscript"
#define MyAppExeName "tech.exe"
#define MyAppStudioName "tech_studio.exe"

[Setup]
AppId={{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}/issues
AppUpdatesURL={#MyAppURL}/releases
DefaultDirName={autopf}\{#MyAppName}
DisableDirPage=no
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=no
OutputBaseFilename=TechScript_v{#MyAppVersion}_x64
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ChangesEnvironment=yes
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
VersionInfoVersion={#MyAppVersion}.0
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription=TechScript Programming Language Installer
VersionInfoCopyright=Copyright (C) 2026 {#MyAppPublisher}
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}

; Process Safety Mutex (Prevents installing over running processes)
AppMutex=TechScriptStudioMutex,TechScriptCliMutex
CloseApplications=yes
RestartApplications=no

; Visual Branding & Assets
SetupIconFile=..\assets\icons\icon.ico
WizardSmallImageFile=..\assets\icons\icon-64.bmp
WizardImageFile=..\assets\icons\wizard.bmp
UninstallDisplayIcon={app}\tech_studio.exe

; License & Privacy Dialogs
LicenseFile=..\LICENSE
InfoBeforeFile=..\PRIVACY.txt

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

; ============================================================
;  FILES — Binaries, Extension, Examples, Docs, Icons
; ============================================================
[Files]
; --- Core Binaries ---
Source: "..\runtime\target\x86_64-pc-windows-msvc\release\tech.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\runtime\target\x86_64-pc-windows-msvc\release\tech.exe"; DestName: "techscript.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\runtime\target\x86_64-pc-windows-msvc\release\tech_studio.exe"; DestDir: "{app}"; Flags: ignoreversion

; --- Icon for shell integration ---
Source: "..\assets\icons\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

; --- VS Code Extension ---
Source: "..\vscode-extension\*"; DestDir: "{%USERPROFILE}\.vscode\extensions\tanmoy.techscript-1.0.7"; Flags: ignoreversion recursesubdirs createallsubdirs

; --- Example Scripts ---
Source: "..\examples\*"; DestDir: "{app}\examples"; Flags: ignoreversion recursesubdirs createallsubdirs

; --- Documentation ---
Source: "..\docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion

; ============================================================
;  CLEANUP — Remove legacy installations and stale shortcuts
; ============================================================
[InstallDelete]
Type: filesandordirs; Name: "{commonprograms}\TechScript"
Type: filesandordirs; Name: "{userprograms}\TechScript"
Type: files; Name: "{commondesktop}\TechScript*.lnk"
Type: files; Name: "{userdesktop}\TechScript*.lnk"
Type: files; Name: "{userdesktop}\tech*.lnk"
; Clean old VS Code extension versions
Type: filesandordirs; Name: "{%USERPROFILE}\.vscode\extensions\techscript-team.techscript-1.0.6"
Type: filesandordirs; Name: "{%USERPROFILE}\.vscode\extensions\tanmoy.techscript-1.0.6.1"
Type: filesandordirs; Name: "{%USERPROFILE}\.vscode\extensions\tanmoy.techscript-1.0.6"
Type: filesandordirs; Name: "{%USERPROFILE}\.vscode\extensions\techscript-1.0.6"
Type: filesandordirs; Name: "{%USERPROFILE}\.vscode\extensions\techscript-team.techscript-1.0.5"

; ============================================================
;  TASKS — User-selectable installation options
; ============================================================
[Tasks]
Name: "desktopicon"; Description: "Create a Desktop shortcut for TechScript Studio"; GroupDescription: "Shortcuts:"
Name: "addtopath"; Description: "Add TechScript to PATH environment variable (recommended)"; GroupDescription: "System Integration:"; Flags: checkedonce
Name: "fileassoc"; Description: "Associate .txs files with TechScript"; GroupDescription: "System Integration:"; Flags: checkedonce

; ============================================================
;  SHORTCUTS — Start Menu & Desktop
; ============================================================
[Icons]
; Studio IDE launcher (no CMD window)
Name: "{group}\TechScript Studio IDE"; Filename: "{app}\tech_studio.exe"; WorkingDir: "{app}"; IconFilename: "{app}\icon.ico"; Comment: "TechScript Studio Visual IDE"
; Development command prompt
Name: "{group}\TechScript Command Prompt"; Filename: "{cmd}"; Parameters: "/k ""set PATH=%PATH%;{app}"""; WorkingDir: "{app}"; Comment: "TechScript Development Command Prompt"
; Documentation link
Name: "{group}\TechScript Documentation"; Filename: "{app}\docs\QUICKSTART.md"; Comment: "TechScript Quick Start Guide"
; Uninstaller
Name: "{group}\Uninstall TechScript"; Filename: "{uninstallexe}"
; Desktop icon (optional task)
Name: "{autodesktop}\TechScript Studio IDE"; Filename: "{app}\tech_studio.exe"; WorkingDir: "{app}"; IconFilename: "{app}\icon.ico"; Comment: "TechScript Studio Visual IDE"; Tasks: desktopicon

; ============================================================
;  REGISTRY — File Associations for .txs and .tx
; ============================================================
[Registry]
; .txs file association
Root: HKA; Subkey: "Software\Classes\.txs"; ValueType: string; ValueName: ""; ValueData: "TechScript.Script"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\.txs"; ValueType: string; ValueName: "Content Type"; ValueData: "text/x-techscript"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\.txs"; ValueType: string; ValueName: "PerceivedType"; ValueData: "text"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script"; ValueType: string; ValueName: ""; ValueData: "TechScript Script File"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script"; ValueType: string; ValueName: "FriendlyTypeName"; ValueData: "TechScript Source File (.txs)"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\icon.ico,0"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\tech.exe"" run ""%1"" --double-click"; Flags: uninsdeletekey; Tasks: fileassoc
; Right-click → "Edit in TechScript Studio"
Root: HKA; Subkey: "Software\Classes\TechScript.Script\shell\edit"; ValueType: string; ValueName: ""; ValueData: "Edit in TechScript Studio"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script\shell\edit"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\tech_studio.exe,0"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Script\shell\edit\command"; ValueType: string; ValueName: ""; ValueData: """{app}\tech_studio.exe"""; Flags: uninsdeletekey; Tasks: fileassoc

; .tx library file association
Root: HKA; Subkey: "Software\Classes\.tx"; ValueType: string; ValueName: ""; ValueData: "TechScript.Library"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Library"; ValueType: string; ValueName: ""; ValueData: "TechScript Library File"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKA; Subkey: "Software\Classes\TechScript.Library\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\icon.ico,0"; Flags: uninsdeletekey; Tasks: fileassoc

[Code]
var
  IsUpdating: Boolean;
  MaintenancePage: TInputOptionWizardPage;
  MaintenanceAction: Integer; // 0 = Modify, 1 = Repair, 2 = Uninstall

function KillProcess(const ImageName: String): Boolean;
var
  ResultCode: Integer;
begin
  Result := Exec('taskkill', '/F /IM "' + ImageName + '"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
end;

function RemoveQuotes(const S: String): String;
var
  Temp: String;
begin
  Temp := S;
  if (Length(Temp) >= 2) and (Temp[1] = '"') and (Temp[Length(Temp)] = '"') then
  begin
    Temp := Copy(Temp, 2, Length(Temp) - 2);
  end;
  Result := Temp;
end;

function GetUninstallString(): String;
var
  UninstStr: String;
begin
  Result := '';
  if RegQueryStringValue(HKEY_LOCAL_MACHINE_64, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString', UninstStr) then
  begin
    Result := UninstStr;
    Exit;
  end;
  if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString', UninstStr) then
  begin
    Result := UninstStr;
    Exit;
  end;
  if RegQueryStringValue(HKEY_CURRENT_USER_64, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString', UninstStr) then
  begin
    Result := UninstStr;
    Exit;
  end;
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString', UninstStr) then
  begin
    Result := UninstStr;
    Exit;
  end;
end;

function InitializeSetup(): Boolean;
var
  PrevPath: String;
begin
  // Terminate any running instances of the runtime or IDE to ensure lock-free file writing
  KillProcess('tech_studio.exe');
  KillProcess('tech.exe');
  KillProcess('techscript.exe');

  // Detect if previous installation exists in Registry (checking both 32-bit and 64-bit hives)
  IsUpdating := False;
  
  if RegValueExists(HKEY_LOCAL_MACHINE_64, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString') then
    IsUpdating := True;
  
  if not IsUpdating and RegValueExists(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString') then
    IsUpdating := True;
  
  if not IsUpdating and RegValueExists(HKEY_CURRENT_USER_64, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString') then
    IsUpdating := True;

  if not IsUpdating and RegValueExists(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{5A0F53F8-B05F-4E4D-8A73-5D7F2A28C8C7}_is1', 'UninstallString') then
    IsUpdating := True;

  Result := True;
end;

procedure InitializeWizard();
begin
  // Create the Maintenance Options page
  MaintenancePage := CreateInputOptionPage(
    wpWelcome,
    'Maintenance Options',
    'Modify, repair, or uninstall TechScript Studio.',
    'Select the action you want to perform and click Next.',
    True, // Exclusive radio buttons
    False // No checkboxes
  );
  
  MaintenancePage.Add('Modify: Change selected tasks (shortcuts, PATH environment integration, and file associations).');
  MaintenancePage.Add('Repair: Reinstall all components, shortcuts, registry keys, and reset environment variables.');
  MaintenancePage.Add('Uninstall: Remove TechScript Studio and all associated files from this computer.');
  
  // Default selection is Modify
  MaintenancePage.SelectedValueIndex := 0;
end;

function ShouldSkipPage(PageID: Integer): Boolean;
begin
  // Skip the maintenance page if we are doing a fresh install (not updating)
  if PageID = MaintenancePage.ID then
  begin
    Result := not IsUpdating;
    Exit;
  end;
  
  if IsUpdating then
  begin
    MaintenanceAction := MaintenancePage.SelectedValueIndex;
    
    if MaintenanceAction = 0 then // Modify
    begin
      // Skip directory selection, program group selection, etc. but keep Tasks page
      if (PageID = wpLicense) or (PageID = wpInfoBefore) or (PageID = wpSelectDir) or (PageID = wpSelectProgramGroup) then
      begin
        Result := True;
        Exit;
      end;
    end
    else if MaintenanceAction = 1 then // Repair
    begin
      // Skip everything directly to installing progress page
      if (PageID = wpLicense) or (PageID = wpInfoBefore) or (PageID = wpSelectDir) or 
         (PageID = wpSelectProgramGroup) or (PageID = wpSelectTasks) then
      begin
        Result := True;
        Exit;
      end;
    end;
  end;
  Result := False;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
var
  ResultCode: Integer;
  UninstallExe: String;
begin
  Result := True;
  
  if CurPageID = MaintenancePage.ID then
  begin
    MaintenanceAction := MaintenancePage.SelectedValueIndex;
    if MaintenanceAction = 2 then // Uninstall
    begin
      // Confirm uninstallation
      if MsgBox('Are you sure you want to completely uninstall TechScript Studio and all of its components?', mbConfirmation, MB_YESNO) = IDYES then
      begin
        UninstallExe := RemoveQuotes(GetUninstallString());
        if UninstallExe = '' then
          UninstallExe := ExpandConstant('{app}\unins000.exe');

        if FileExists(UninstallExe) then
        begin
          // Launch the uninstaller
          if Exec(UninstallExe, '/SILENT', '', SW_SHOW, ewNoWait, ResultCode) then
          begin
            WizardForm.Close;
            Result := False; // Prevents going to next page
            Exit;
          end
          else
          begin
            MsgBox('Failed to launch uninstaller: ' + UninstallExe, mbError, MB_OK);
          end;
        end
        else
        begin
          MsgBox('Uninstaller not found. You can uninstall it from Settings -> Apps.', mbError, MB_OK);
        end;
      end;
      Result := False; // Stay on the page if cancelled or failed
    end;
  end;
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if (CurPageID = wpWelcome) and IsUpdating then
  begin
    WizardForm.WelcomeLabel1.Caption := 'TechScript Studio Maintenance';
    WizardForm.WelcomeLabel2.Caption := 'An existing TechScript installation was detected.' + #13#10#13#10 +
      'Click Next to choose maintenance options, including modifying components, repairing the install, or uninstalling.';
  end;
end;

procedure AddPathToFront();
var
  Path: String;
  AppPath: String;
  P: Integer;
begin
  AppPath := ExpandConstant('{app}');
  
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
  begin
    Path := '';
  end;

  // Remove all existing instances to prevent duplicates
  P := Pos(';' + AppPath, Path);
  while P > 0 do
  begin
    Delete(Path, P, Length(AppPath) + 1);
    P := Pos(';' + AppPath, Path);
  end;
  
  P := Pos(AppPath + ';', Path);
  while P > 0 do
  begin
    Delete(Path, P, Length(AppPath) + 1);
    P := Pos(AppPath + ';', Path);
  end;
  
  P := Pos(AppPath, Path);
  while P > 0 do
  begin
    Delete(Path, P, Length(AppPath));
    P := Pos(AppPath, Path);
  end;

  // Prepend to front of PATH for priority
  if Path = '' then
  begin
    Path := AppPath;
  end
  else
  begin
    Path := AppPath + ';' + Path;
  end;

  RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
end;

procedure RemovePath();
var
  Path: String;
  AppPath: String;
  P: Integer;
begin
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
  begin
    AppPath := ExpandConstant('{app}');
    
    P := Pos(';' + AppPath, Path);
    while P > 0 do
    begin
      Delete(Path, P, Length(AppPath) + 1);
      P := Pos(';' + AppPath, Path);
    end;
    
    P := Pos(AppPath + ';', Path);
    while P > 0 do
    begin
      Delete(Path, P, Length(AppPath) + 1);
      P := Pos(AppPath + ';', Path);
    end;
    
    P := Pos(AppPath, Path);
    while P > 0 do
    begin
      Delete(Path, P, Length(AppPath));
      P := Pos(AppPath, Path);
    end;

    RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
  end;
end;

procedure CurUninstallStepChanged(JustAfterAnUninstallStep: TUninstallStep);
begin
  if JustAfterAnUninstallStep = usPostUninstall then
  begin
    RemovePath();
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    if WizardIsTaskSelected('addtopath') then
    begin
      AddPathToFront();
    end;
    
    MsgBox('TechScript v1.0.7 Installation Complete!' + #13#10 + #13#10 +
           'What''s installed:' + #13#10 +
           '  - tech.exe — CLI runtime & REPL' + #13#10 +
           '  - tech_studio.exe — Visual Studio IDE' + #13#10 +
           '  - VS Code extension for .txs syntax' + #13#10 + #13#10 +
           'Please restart your terminal for PATH changes.' + #13#10 +
           'Restart VS Code to activate the extension.',
           mbInformation, MB_OK);
  end;
end;

// ============================================================
//  POST-INSTALL — Launch Studio IDE
// ============================================================
[Run]
Filename: "{app}\tech_studio.exe"; Description: "{cm:LaunchProgram,TechScript Studio IDE}"; Flags: postinstall nowait skipifsilent
Filename: "{app}\docs\QUICKSTART.md"; Description: "View Quick Start Guide"; Flags: postinstall nowait skipifsilent shellexec unchecked
