@echo off
:: Se for chamado com subcomandos diretos do Docker (compose, ps, exec, cp, build, run, stop, start, logs)
:: repassa diretamente para o executavel oficial docker.exe evitando interceptacao
if /i "%1"=="compose" ( docker.exe %* & exit /b %ERRORLEVEL% )
if /i "%1"=="ps" ( docker.exe %* & exit /b %ERRORLEVEL% )
if /i "%1"=="exec" ( docker.exe %* & exit /b %ERRORLEVEL% )
if /i "%1"=="cp" ( docker.exe %* & exit /b %ERRORLEVEL% )
if /i "%1"=="inspect" ( docker.exe %* & exit /b %ERRORLEVEL% )
if /i "%1"=="images" ( docker.exe %* & exit /b %ERRORLEVEL% )

call "%~dp0scripts\docker.bat" %*
