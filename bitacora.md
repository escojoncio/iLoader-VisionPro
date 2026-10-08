# Bitácora — iLoader-VisionPro

Fork de iloader (nab138) con el soporte Wi-Fi de Apple Vision Pro de la rama `visionos-tunnel` de rebelancap, aplicado sobre el iloader oficial 2.3.5 y compilado para Windows.

## Ramas
- `visionos-windows`: **rama de trabajo** (Vision Pro sobre iloader 2.3.5). Último commit funcional: `e2fc39e` (publica el instalador de Windows como GitHub Release).
- `visionos-tunnel-windows`: build anterior sobre la base de rebelancap (histórico).
- `main`: iloader oficial + README del fork; no lleva el código de Vision Pro.

## Estado tras la última build (`visionos-windows`)
Funciona:
- Emparejamiento y sideload al Vision Pro por Wi-Fi desde Windows.
- Increased Memory Limit (`com.apple.developer.kernel.increased-memory-limit`) solicitado en todas las App IDs al firmar, por cualquier vía (IPA propio, SideStore, LiveContainer). Si Apple lo rechaza, instala igual.
- *Place*/exportar emparejamiento como `rp_pairing_file.plist` (nombre que carga SideStore en visionOS), además del nombre de iOS.
- Perfil de aprovisionamiento renovado si no incluye el UDID del dispositivo (error 0xe8008015 al reutilizar una App ID creada desde el iPhone).
- Correcciones: pilas de hilo más grandes en Windows (el emparejamiento cerraba la app); un pair-verify reseteado = emparejamiento rechazado → se descarta y se vuelve a pedir el código.

## Cambios por archivo (rama `visionos-windows`)
- `src-tauri/vendor/isideload/src/sideload/sideloader.rs`: al firmar para un dispositivo, `profile_for_device`: si el perfil descargado no lista el UDID, `delete_provisioning_profile` y nueva descarga; log de dispositivos y plataformas del perfil.
- `src-tauri/vendor/isideload/src/dev/app_ids.rs`: borrado de perfiles (`deleteProvisioningProfile`).
- `src-tauri/src/sideload.rs`: `set_target_device` antes de firmar.
- Emparejamiento visionOS (`69142a6`, `9fccdba`): nombre `rp_pairing_file.plist`; reset durante pair-verify tratado como rechazo.
- `.github/workflows/build.yml`: compila solo con `[build]` en el commit o *Run workflow*; sin job programado de descargas; publica el instalador de Windows como Release.

## Notas
- El ID de equipo de las App IDs registradas por SideStore/iloader tiene la forma `<bundle>.<TEAMID>`.
- Lo equivalente para instalar desde el propio Vision Pro está en SideStoreRAM-VisionOS-Compatible (parche gemelo de renovación de perfil).

## Pendiente
- Nada bloqueante. Si se actualiza iloader oficial, rebasar `visionos-windows` y volver a compilar con `[build]`.
