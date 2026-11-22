# Bitácora — iLoader-VisionPro

Fork de iloader (nab138) con el soporte Wi-Fi de Apple Vision Pro de la rama `visionos-tunnel` de rebelancap, aplicado sobre el iloader oficial 2.3.5 y compilado para Windows.

## Ramas
- `visionos-windows`: **rama de trabajo** (Vision Pro sobre iloader 2.3.5). Último commit funcional: `1192b17` (publica el instalador de Windows como GitHub Release).
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
- Emparejamiento visionOS (`c5f5e37`, `2333fac`): nombre `rp_pairing_file.plist`; reset durante pair-verify tratado como rechazo.
- `.github/workflows/build.yml`: compila solo con `[build]` en el commit o *Run workflow*; sin job programado de descargas; publica el instalador de Windows como Release.

## Notas
- El ID de equipo de las App IDs registradas por SideStore/iloader tiene la forma `<bundle>.<TEAMID>`.
- Lo equivalente para instalar desde el propio Vision Pro está en SideStoreRAM-VisionOS-Compatible (parche gemelo de renovación de perfil).

## Pendiente
- Nada bloqueante. Si se actualiza iloader oficial, rebasar `visionos-windows` y volver a compilar con `[build]`.

## 2026-10-19 — iloader 2.3.10
- Aplicado el diff upstream v2.3.8..v2.3.10 (`git apply -3`, sin conflictos): versión 2.3.10 en package.json, src-tauri/Cargo.toml, Cargo.lock (solo entrada iloader), tauri.conf.json; `src/AppleID.tsx`, `src/ErrorContext.tsx`, `src/components/OperationView.tsx`, `src/pages/Settings.tsx` (clave de ajustes `newAnisetteServer`, servidor por defecto `new.ani.sidestore.io` identidad Mac; servidores Windows al final, poco fiables y sin SMS); `src/locales/zh_tw.json`.
- Sin cambios en isideload/idevice vendorizados: el estado de anisette se reprovisiona solo con -45061 al cambiar de servidor; `reset_anisette_state` sigue disponible.
- `src-tauri/tests/gsa_signin.rs` y `STATUS.md`: eliminado un correo de tercero.
- Pendiente: probar login (Mac y Windows) con la build y el emparejamiento Vision Pro.

## 2026-11-22 — anisette (error 503), sin compilar ni probar
- Lista de servidores (`src/pages/Settings.tsx`): `new.ani.sidestore.io` (defecto, primero), `ani.iloader.app`, `mac-ani.jkcoxson.com`, `ani3.stikstore.app`. Servidores Windows (`ani.jkcoxson.com`, `ani2.stikstore.app`) comentados, como en upstream.
- Clave del store `newAnisetteServer` → `anisetteServer3-1` en `Settings.tsx`, `AppleID.tsx`, `ErrorContext.tsx`, `components/OperationView.tsx`: descarta servidores guardados antiguos (cambio de comportamiento: los ajustes previos del usuario se pierden).
- Sin tocar: `account.rs` (client_info ya igual al estado final de upstream tras 7364ae6), isideload vendorizado 0.4.6 (upstream 7364ae6 usa 0.5.2; no rebasado), `vision.rs`, emparejamiento y login Vision Pro. El flag `increased_memory_limit` de d319ce7 ya lo cubre `sideload.rs` (`INCREASED_MEMORY_LIMIT`).
- Unverified: que 503 desaparezca (puede ser caída del propio servidor); `tsc` no ejecutado (sin node_modules); `ani.iloader.app` sin probar. Revisión adversarial: limpia.
- Pendiente: compilar con `[build]` y probar login; decidir si rebasar a isideload 0.5.2 (requiere rehacer parches de vendor).

## 2026-11-22 (2) — 412 al verificar código SMS 2FA: SIN resolver, sin compilar ni probar
- Síntoma: tras introducir el código SMS, `verify_sms_2fa` (`src-tauri/vendor/isideload/src/auth/apple_account.rs`) recibe HTTP 412 con el cuerpo del reto (sin `serviceErrors`, `success:0`, flags de bloqueo a false) y el login aborta. Un intento previo (208e31c, build-21) trataba ese 412 como 'código incorrecto' y volvía a pedir el código: REVERTIDO porque el 412 persiste con códigos correctos y el bucle podía bloquear la cuenta de Apple. Un 412 en verify aborta a la primera.
- Queda solo diagnóstico: `warn!` en verify con status, id de teléfono, longitud del código y cuerpo censurado (`censor_sms_body`: email, `prs_account_nm=`, teléfonos). Ojo: `parse_sms_error` sigue incluyendo el cuerpo SIN censurar en el mensaje de error.
- Causa raíz no confirmada. Upstream isideload HEAD (326e562, 0.5.2) tiene el mismo `verify_sms_2fa` y tampoco trata el 412 en verify; SideSign tampoco. Hipótesis sin probar: identidad/cabeceras anisette distintas según servidor (`X-Mme-Client-Info`, `X-Xcode-Version` fijo `27.0 (27A5218g)`), falta de cookie store en reqwest (SideSign/URLSession sí usa cookies). Siguiente paso: probar con otro servidor anisette (`ani.iloader.app`, `mac-ani.jkcoxson.com`) y capturar el cuerpo del 412 del log.
- La UI nunca tuvo selector de método SMS/dispositivo/número (solo un TODO en aeec5b2, borrado en e353f02); el método lo fija Apple (`authType`). Pendiente si se quiere: selector en `src/AppleID.tsx` + `src-tauri/src/account.rs` (el evento `2fa-required` ya lleva `numbers`).
