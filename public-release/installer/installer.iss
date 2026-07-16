; TechScript 2.0 Installer Script (Inno Setup)
[Setup]
AppName=TechScript 2.0
AppVersion=2.0.0
DefaultDirName={autopf}\TechScript
DefaultGroupName=TechScript 2.0
UninstallDisplayIcon={app}\bin\tsc.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=TechScript_Setup
SetupIconFile=C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\TechScript-Logo-Package\logo-package\windows\installer-icon.ico

[Files]
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\bin\*"; DestDir: "{app}\bin"; Flags: recursesubdirs createallsubdirs
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\stdlib\*"; DestDir: "{app}\stdlib"; Flags: recursesubdirs createallsubdirs
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\examples\*"; DestDir: "{app}\examples"; Flags: recursesubdirs createallsubdirs
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\docs\*"; DestDir: "{app}\docs"; Flags: recursesubdirs createallsubdirs
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\vscode\*"; DestDir: "{app}\vscode"; Flags: recursesubdirs createallsubdirs
Source: "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\public-release\portable\TechScript\templates\*"; DestDir: "{app}\templates"; Flags: recursesubdirs createallsubdirs

[Registry]
; Register .txs file association
Root: HKA; Subkey: "Software\Classes\.txs"; ValueType: string; ValueName: ""; ValueData: "TechScriptFile"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\TechScriptFile"; ValueType: string; ValueName: ""; ValueData: "TechScript Source File"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\TechScriptFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\bin\file-icon.ico"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\TechScriptFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\bin\tsc.exe"" ""%1"""; Flags: uninsdeletekey

; Add bin directory to user path environment variable
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: NeedsAddPath

[Code]
function NeedsAddPath(): Boolean;
var
  OldPath: String;
begin
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
  begin
    Result := Pos('TechScript\bin', OldPath) = 0;
  end
  else
    Result := True;
end;
