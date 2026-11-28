<h1 align="center">Quiet.md</h1>

<p align="center">
  <b>Notas rápidas y tranquilas que viven en tu ordenador.</b><br>
  Una app de notas nativa hecha con Rust + GPUI: carpetas de verdad, Markdown que se formatea solo y una pizarra dentro de tus notas. Gratis y de código abierto.
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · Español · <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <a href="https://github.com/s1dashu/quiet-md/releases/latest"><img src="https://img.shields.io/github/v/release/s1dashu/quiet-md?label=release&color=2f6feb" alt="Última versión"></a>
  <a href="https://github.com/s1dashu/quiet-md/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/s1dashu/quiet-md/ci.yml?branch=main&event=schedule&label=CI" alt="Estado de CI"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="macOS, Windows y Linux">
  <img src="https://img.shields.io/badge/built%20with-Rust%20%2B%20GPUI-dea584?logo=rust" alt="Hecha con Rust y GPUI">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/s1dashu/quiet-md?color=green" alt="Licencia MIT"></a>
  <img src="https://img.shields.io/badge/status-beta-orange" alt="Beta">
</p>

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="Descargar para macOS"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Descargar para Windows"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Descargar para Linux"></a>
</p>

<p align="center">
  <a href="https://quiet.md/?lang=en"><b>Sitio web</b></a> ·
  <a href="#descarga"><b>Descarga</b></a> ·
  <a href="#funciones">Funciones</a> ·
  <a href="#rendimiento">Rendimiento</a> ·
  <a href="CONTRIBUTING.md">Contribuir</a>
</p>

<p align="center">
  <img src="docs/images/readme-home.webp" width="100%" alt="Quiet.md sobre una pradera en pixel art: la barra lateral muestra las carpetas y la página de la carpeta principal presenta cada carpeta como una pila de tarjetas de notas con imágenes.">
</p>

> [!NOTE]
> Quiet.md está en **beta** y cambia rápido. Si encuentras un error o tienes una idea, [abre un issue](https://github.com/s1dashu/quiet-md/issues), o mejor aún, envía un pull request.

## Por qué Quiet.md

**📁 Tus notas son archivos, sin más.** Sin cuenta, sin nube y, por defecto, nada sale a la red (salvo la comprobación opcional de actualizaciones, los recuentos de uso anónimos que la app y la herramienta de línea de comandos `quiet` envían una vez al día —solo recuentos, como cuántas veces se abrió la app; se desactivan en Ajustes › General › Estadísticas de uso anónimas— y las fuentes que tú provocas: la manuscrita para chino, japonés y coreano la primera vez que una pizarra la necesita, y una fuente de documento descargable cuando la eliges). Cada carpeta de la barra lateral es una carpeta real de tu disco, y cada nota es un archivo con el nombre de su título: `.md` para Markdown y `.xml` para documentos enriquecidos. Sincronízalas con iCloud, Git o lo que prefieras. Si algún día dejas Quiet.md, tus archivos siguen exactamente donde estaban.

**⚡ Nativa, y rápida.** Toda la interfaz la dibujan en la GPU Rust y [GPUI](https://www.gpui.rs/). Sin Electron, sin WebView. En una nota de un millón de caracteres, escribir cuesta unos 5 ms por fotograma, y una pantalla de 120 Hz da 8,3.

**✍️ Escribe y dibuja en el mismo sitio.** Teclea `#`, `-`, `[]` o `>` y el Markdown se formatea solo. Cuando las palabras no bastan, pon una pizarra dentro de la nota. Las notas enriquecidas se guardan en el formato DocxXML de Feishu (Lark), así que al pegar desde Feishu o desde una página web se conserva la estructura.

## Funciones

### Escribir

<p align="center">
  <img src="docs/images/readme-write.webp" width="100%" alt="Una nota larga abierta en Quiet.md, con encabezados, fórmulas en línea y una tabla de concentraciones de iones.">
</p>

Encabezados, listas, tareas, citas, avisos destacados, columnas, tablas con celdas combinadas, código con resaltado de sintaxis, fórmulas LaTeX, imágenes, archivos y portadas. Los atajos de Markdown funcionan mientras escribes y el menú `/` tiene todo lo demás. Menciona otra nota para enlazarla; esa nota mostrará sus referencias inversas. Las bases de datos guardan registros con campos de selección, fecha, número, fórmula, búsqueda y relación, en vistas que filtras, ordenas y agrupas; cada celda de una columna Notas es una nota de verdad. El menú … tiene Modo concentración, Modo máquina de escribir y Enfoque de párrafo, y Visualización › Acercar agranda solo el texto.

### Dibujar

<p align="center">
  <img src="docs/images/readme-board.webp" width="100%" alt="Una pizarra en Quiet.md: imágenes y mapas dispuestos con etiquetas manuscritas, cajas y flechas que los unen.">
</p>

Las pizarras son un tablero portado de Excalidraw: formas, flechas que siguen unidas, trazo libre, texto, notas adhesivas, marcos e imágenes. Ponla dentro de una nota o guárdala como archivo `.excalidraw`, que excalidraw.com abre directamente. Haz zoom pellizcando en el trackpad; una pizarra con 10.000 formas sigue fluida.

### Organiza en carpetas de verdad

La barra lateral agrupa tus carpetas de primer nivel; al abrir una, su página muestra las subcarpetas como pilas de sus páginas y la vista de lista ordena por cualquier columna. Añade cualquier carpeta del ordenador con **Añadir carpeta** y se usa donde está. Mantén una nota sobre una carpeta y se abrirá sola; las carpetas pueden ir dentro de otras carpetas, las páginas y carpetas pasan de una carpeta de primer nivel a otra, y cada movimiento se puede deshacer. Fija notas arriba en la barra lateral, busca en todos los títulos y líneas desde el icono de búsqueda o con `⌘ J`, y busca y reemplaza dentro de una nota, también con expresiones regulares. Las notas eliminadas esperan en la papelera los días que elijas. Un archivo Markdown abierto por separado se edita en su propia ventana, donde está.

### Trae tus notas

Importar desde otra app… (en el menú Archivo) busca en el ordenador tus bibliotecas de Obsidian, Logseq, iA Writer y Typora y tus espacios de trabajo de SiYuan, y los copia sin tocar los originales. Los `[[enlaces wiki]]` de Obsidian siguen funcionando: haz clic en uno para abrir la nota a la que apunta. Más adelante, arrastra archivos y carpetas Markdown a la página de una carpeta o directamente a una carpeta de la barra lateral. Las notas vuelven a salir como Markdown o PDF, o directas a la impresora.

### Y además

- **Historial de versiones**: cada nota guarda instantáneas al guardarse (20 por defecto, hasta 30). Abre una para ver en su sitio qué cambió, restáurala con un clic o ponle nombre.
- **Notas rápidas**: en macOS, un atajo global (`⌥ Espacio` por defecto) abre una pequeña ventana de nota sobre cualquier app.
- **Tu aspecto**: cuatro estilos integrados —Craft, Feishu, Notion y Obsidian—, cada uno con variante oscura, o tu propio CSS ([docs/THEMES.md](docs/THEMES.md)). Elige la fuente de la interfaz y la de los documentos.
- **Tus teclas**: pulsa `⌘ /` para ver todos los atajos y reasignar o borrar cualquiera. En Windows y Linux, Ctrl sustituye a ⌘ y Alt a ⌥.
- **Línea de comandos y agentes**: la herramienta `quiet`, incluida en la app, permite que agentes de IA como Claude Code o Codex lean y editen tu biblioteca con el mismo motor que la app. Ajustes › Conector de agentes te da una línea para pegarle a tu agente, que se instala solo; en macOS instala la herramienta con un clic. Más detalles en el [README en inglés](README.md#work-with-agents).
- **Tu idioma**: inglés, chino simplificado, chino tradicional, japonés, coreano, español, francés, árabe y turco. El árabe une sus letras y se escribe de derecha a izquierda.
- **Limpieza**: Ajustes › Biblioteca encuentra las imágenes y archivos que ya no usa ninguna nota y los retira cuando lo confirmas.
- **Actualizaciones**: las versiones nuevas se instalan desde la propia app tras verificar su firma. Las comprobaciones automáticas se pueden desactivar.

## Hoja de ruta

- [x] Documentos enriquecidos: encabezados, listas, avisos destacados, tablas, columnas, código, fórmulas, imágenes y archivos
- [x] Archivos Markdown, abiertos y editados donde están
- [x] Pizarras: diagramas y bocetos a mano alzada, dentro de las notas o por separado
- [x] Abre cualquier carpeta donde está, una junto a otra en la barra lateral
- [x] Importa desde Obsidian, Logseq, SiYuan, Typora, iA Writer y cualquier carpeta de Markdown
- [x] Modos de escritura: concentración, máquina de escribir y enfoque de párrafo
- [x] Temas, modo oscuro y nueve idiomas
- [x] Una CLI `quiet` y una skill para agentes de IA
- [ ] Bases de datos (en curso)
- [ ] Diagramas Mermaid (en curso)
- [ ] Móviles: iPhone, iPad y Android (en curso)
- [ ] Sincronización entre dispositivos (en curso)
- [ ] Propiedades de documento
- [ ] Recortes web

## Descarga

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="Descargar para macOS"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Descargar para Windows"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Descargar para Linux"></a>
</p>

| Plataforma | Paquete | Requisitos |
|---|---|---|
| **macOS** (Apple Silicon) | [quiet-md-macos-arm64.dmg](https://download.quiet.md/latest/quiet-md-macos-arm64.dmg) | macOS 13 o posterior |
| **Windows** (x64) | [quiet-md-windows-x64-setup.exe](https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe) | Windows 10 versión 1809 o posterior |
| **Linux** (x86_64, experimental) | [quiet-md-x86_64.AppImage](https://download.quiet.md/latest/quiet-md-x86_64.AppImage) | glibc 2.35 o posterior (Ubuntu 22.04 o más reciente) |
| **macOS** (Intel) | [quiet-md-macos-x86_64.dmg](https://download.quiet.md/latest/quiet-md-macos-x86_64.dmg) | macOS 13 o posterior |

- **macOS**: abre el `.dmg`, arrastra `quiet.md.app` al acceso directo de Aplicaciones que hay al lado y ábrela desde Aplicaciones. La app y la imagen de disco están firmadas y notarizadas por Apple. El `.zip` de la página de Releases contiene la misma app; es lo que descargan las actualizaciones integradas.
- **Windows**: ejecuta el instalador. Por defecto se instala solo para el usuario actual, y desinstalarla nunca borra tus notas. Si aparece SmartScreen en el primer arranque, haz clic en **Más información → Ejecutar de todas formas**.
- **Linux** (experimental): `chmod +x quiet-md-*.AppImage` y ejecútala; sin FUSE, añade `--appimage-extract-and-run`. La AppImage admite actualizaciones desde la app a partir de la 0.1.10; las versiones de Linux anteriores necesitan una instalación manual una sola vez. Por ahora solo se ha probado a mano en Omarchy con Wayland; consulta las [notas para Linux](CONTRIBUTING.md#linux-experimental).

También puedes descargarla desde [Quiet.md](https://quiet.md/?lang=en). Todas las versiones y sus sumas SHA-256 están en [GitHub Releases](https://github.com/s1dashu/quiet-md/releases).

En el primer arranque, Quiet.md abre directamente una carpeta **Quiet Notes** en tus Documentos (o la carpeta `Quiet.md` que creó una versión anterior) con la guía **Getting started**: un recorrido de diez minutos por las funciones principales. Nunca sobrescribe las notas que ya tengas. Añade otras carpetas o trae notas de otras apps cuando quieras, desde el menú Archivo.

## Rendimiento

Tiempo de procesamiento por fotograma a lo largo de 100 operaciones seguidas en un documento mixto de **1 millón de caracteres** y 549 capítulos:

| Operación | P50 | P95 |
|---|---:|---:|
| ⌨️ Escribir | **5.06 ms** | 5.78 ms |
| ⌫ Retroceso | **4.99 ms** | 5.55 ms |
| 🖱️ Desplazarse | **2.08 ms** | 4.14 ms |

Una pantalla de 120 Hz da 8.3 ms a cada fotograma. Incluso en el P95 de un documento de un millón de caracteres sobra margen.

| | |
|---|---|
| Descarga para macOS | ~18 MB |
| Instalador para Windows | ~14 MB |
| En ejecución | Sin WebView, sin Electron, sin servicios en segundo plano |

<sub>Apple M5 Pro / 48 GiB, macOS 26.5, compilación release. Metodología y scripts para reproducirlo: [docs/PERFORMANCE.md](docs/PERFORMANCE.md).</sub>

## Colaboradores

Gracias a todas las personas que han ayudado a construir Quiet.md. Los issues y pull requests son bienvenidos: cómo compilar, probar y las convenciones de código están en [CONTRIBUTING.md](CONTRIBUTING.md).

[![Colaboradores](https://raw.githubusercontent.com/s1dashu/quiet-md/contributors-assets/docs/images/contributors.svg?v=1)](https://github.com/s1dashu/quiet-md/graphs/contributors)

## Comunidad

Si usas WeChat, escanea el código de abajo para unirte a nuestro grupo de usuarios. Pronto abriremos también un servidor de Discord.

<img src="docs/images/wechat-group.webp" alt="Grupo de usuarios de WeChat" width="200">

## Novedades recientes

- **0.2.0** (28 nov): una interfaz más sencilla sobre un único sistema de diseño, bases de datos para todos, tres modos de escritura y una línea de comandos que tu agente de IA puede usar.
  - **Las carpetas son el nivel superior**: ya no hay una página de Inicio aparte. La barra lateral agrupa las carpetas de nivel superior bajo encabezados plegables, y el menú … de cada encabezado tiene Gestionar, Mostrar en el Finder y Quitar de la barra lateral. Añadir carpeta usa la carpeta donde está. Mis documentos y Archivos de este ordenador desaparecen: un archivo Markdown abierto por separado tiene su propia ventana, y las carpetas que abriste antes pasan a ser carpetas de nivel superior. Arrastra una página o carpeta a otra carpeta de nivel superior, o usa Mover a; sus imágenes, enlaces e historial de versiones van con ella, y se puede deshacer. Favoritos se integra en Fijados.
  - **Un único sistema de diseño**: menús, botones, diálogos, interruptores, descripciones emergentes y barras de desplazamiento son un solo conjunto de componentes basado en shadcn/ui, con esquinas continuas (squircle) en macOS, un anillo fino y una sombra suave alrededor de los elementos emergentes, una sola escala de sombras e iconos de Nucleo en todas partes. Los menús que no caben se desplazan, y al ir hacia un submenú ya no se iluminan las filas que cruzas. La cabecera muestra + · Buscar · … en todas las páginas (+ · Mostrar · Buscar · … en las carpetas), y la ruta se alinea con los botones de la ventana por la altura de las mayúsculas.
  - **Bases de datos**: créalas desde el menú Nuevo, el menú `/` o un archivo CSV. Hay campos de texto, selección única y múltiple, fecha, número, fórmula, búsqueda y vínculo; las vistas filtran, ordenan, agrupan y colorean filas según reglas, con una altura de fila Ajustar al contenido. Cada celda de una columna Nota es una nota real de tu biblioteca.
  - **Modos de escritura**, en el menú … del documento: Modo concentración (⌘.) deja el documento solo en su ventana; Modo máquina de escribir (⌘⇧T) mantiene en el centro la línea que escribes; Enfoque de párrafo (⌘⇧F) deja nítido el párrafo actual y atenúa el resto. Mostrar › Acercar / Alejar (⌘+ / ⌘−) escala solo el texto de la nota, y con él los títulos, tablas, código y ecuaciones.
  - **Línea de comandos y agentes**: la nueva herramienta `quiet` lista, busca, lee y edita notas y bases de datos desde un terminal, añade y quita carpetas de la barra lateral y abre un documento en un bloque concreto. Ajustes › Conector de agentes configura Claude Code, Codex y otros agentes con un solo comando; `quiet` viene con la app y se añade a tu PATH (Instalar comando en macOS, el instalador en Windows).
  - **Un modo oscuro más legible**: enlaces, colores de texto, resaltados, fechas de recordatorio, cuentas atrás y textos de botones alcanzan un contraste legible; la barra lateral es más luminosa, y ⌃⌘D alterna entre oscuro y claro.
  - **Además**: vuelven las subpáginas; al pegar desde Word, navegadores o Feishu se conserva el formato en Windows y Linux; importa espacios de trabajo de SiYuan, con SiYuan, Obsidian, Typora y carpetas Markdown reunidos en Importar desde otra app…; la Papelera se vacía sola tras 1 a 30 días; cualquier atajo se puede borrar; Abrir en una ventana nueva sigue el tamaño y la barra lateral de la ventana principal; renombrar o mover una carpeta se ve al instante mientras los archivos la siguen en segundo plano; el primer arranque abre directamente Documentos › Quiet Notes y las guías reagrupadas, con archivos de ejemplo de Markdown, pizarra y base de datos; recuento anónimo de uso diario, que puedes desactivar en Ajustes › General.
  - **Formato de la biblioteca**: al abrir una biblioteca se actualiza a FORMAT 6, y después la 0.1.14 y anteriores ya no pueden abrirla. Las bibliotecas en iCloud y otras carpetas sincronizadas no se actualizan por ahora.
- **0.1.14** (22 nov): buscar y reemplazar con expresiones regulares; los `[[enlaces wiki]]` de Obsidian abren su nota; interfaces en turco y en árabe de derecha a izquierda; elige las fuentes de la interfaz y de los documentos; limpia imágenes y archivos sin usar; búsqueda y borrado masivo más rápidos en bibliotecas grandes.
- **0.1.13** (17 nov): pizarras, portadas de Excalidraw, dentro de una nota o como archivo `.excalidraw`; historial de versiones en cada nota; interfaces en francés y árabe; Copiar como Markdown; las imágenes pegadas desde la web se guardan en la biblioteca.
- **0.1.12** (14 nov): arrastra archivos desde el Finder directamente a las carpetas de la barra lateral; tarjetas de Inicio con esquinas continuas; Abrir en el Finder; un ancho mínimo de ventana que deja sitio a la nota.
- **0.1.11** (13 nov): la biblioteca pasa a ser carpetas reales con archivos nombrados por su título, migrada automáticamente con copia de seguridad; un nuevo primer arranque con importación desde Obsidian; interfaces en japonés, español y coreano; fórmulas en bloque en Markdown; arranque más rápido y menos memoria.
- **0.1.10** (10 nov): elige un tipo de documento en el menú Nuevo; las tablas anchas se desplazan en horizontal; Markdown conserva la alineación de columnas.
- **0.1.9** (8 nov): correcciones para Cmd+Q en macOS, los botones de ventana en Windows y las selecciones ocultas por los resaltados.
- **0.1.8** (9 nov): edita archivos Markdown locales en su sitio; carpetas anidadas con arrastre y deshacer; menos memoria para imágenes; un AppImage experimental para Linux.
- **0.1.7** (6 nov): guardado en varias ventanas y recuperación de conflictos más seguros; las exportaciones a Markdown conservan los adjuntos en línea.
- **0.1.6** (6 nov): escritura y cambio de ventana más fluidos, tablas más anchas, ⌥ Space para notas rápidas en macOS; pegar imágenes, exportar a PDF e imprimir en Windows.
- **0.1.5** (5 nov): pasa a llamarse Quiet.md; gestiona varios documentos a la vez; vacía la Papelera de una vez.
- **0.1.4** (5 nov): actualizaciones desde la app, verificadas por firma; modo oscuro en todos los temas.
- **0.1.3** (4 nov): texto de la barra lateral legible cuando macOS está en modo oscuro.
- **0.1.2** (4 nov): los menús y atajos estándar de macOS; el icono del Dock vuelve a abrir una ventana.
- **0.1.1** (3 nov): primera versión para Windows x64.

Las notas completas de cada versión están en [Releases](https://github.com/s1dashu/quiet-md/releases).

## Licencia

El código se publica bajo la licencia [MIT](LICENSE). El código, las fuentes y los recursos de terceros se enumeran en [THIRD_PARTY.md](THIRD_PARTY.md).

Lo siguiente no está cubierto por la licencia MIT y debe sustituirse en los forks:

- Los iconos de Nucleo de `assets/icons/` (82) y `assets/editor-icons/` (24) están bajo una licencia comercial de Nucleo. No los extraigas ni los redistribuyas por separado.
- El nombre “Quiet.md” y el icono de la app.

## Star History

<a href="https://www.star-history.com/#s1dashu/quiet-md&type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date&theme=dark" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date" />
  </picture>
</a>
