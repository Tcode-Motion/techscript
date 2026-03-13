[Version]
Class=IEXPRESS
SEDVersion=3
[Options]
PackagePurpose=InstallApp
ShowInstallProgramWindow=0
HideExtractAnimation=0
UseLongFileName=0
InsideCompressed=0
CAB_FixedSize=0
CAB_ResvCodeSigning=0
RebootMode=N
InstallPrompt=%InstallPrompt%
DisplayLicense=%DisplayLicense%
FinishMessage=%FinishMessage%
TargetName=%TargetName%
FriendlyName=%FriendlyName%
AppLaunched=%AppLaunched%
PostInstallCmd=%PostInstallCmd%
AdminQuietInstCmd=%AdminQuietInstCmd%
UserQuietInstCmd=%UserQuietInstCmd%
SourceFiles=SourceFiles
[Strings]
InstallPrompt=Do you want to install TechScript v1.0.4?
DisplayLicense=
FinishMessage=TechScript v1.0.4 has been extracted to C:\TechScript. Please add C:\TechScript to your system PATH to use it globally.
TargetName=c:\Users\tanmoy\Documents\jast work on this now\techscript\public-release\TechScript_v1.0.4_Setup.exe
FriendlyName=TechScript v1.0.4 Setup
AppLaunched=cmd.exe /c "mkdir C:\TechScript & copy /Y techscriptv1.0.4.exe C:\TechScript\techscript.exe & echo Installation complete!"
PostInstallCmd=<None>
AdminQuietInstCmd=
UserQuietInstCmd=
[SourceFiles]
SourceFiles0=c:\Users\tanmoy\Documents\jast work on this now\techscript\public-release\bin\
[SourceFiles0]
%FILE0%=
[Strings]
FILE0="techscriptv1.0.4.exe"
