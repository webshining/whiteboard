use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@excalidraw/excalidraw@0.18.0/dist/prod/index.min.css">
            <title>Whiteboard</title>
            <styles>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }

                body {
                    width: 100dvw;
                    height: 100dvh;

                    background: #3c3c3c;
                }

                .content__wrapper {
                    width: 100%;
                    height: 100%;

                    display: flex;
                    justify-content: center;
                    align-items: center;

                    padding: 10px;
                }

                .content {
                    width: 100%;
                    height: 100%;

                    border-radius: 15px;
                    border: 3px solid #4e4e4e;
                    background: #313131;
                }
            </styles>
        </head>
        <body>
            <div class="content__wrapper">
                <div class="content"></div>
            </div>

            <script src="https://unpkg.com/react@18/umd/react.production.min.js"></script>
            <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/@excalidraw/excalidraw@0.18.0/dist/prod/index.min.js"></script>
            <script src="https://cdn.socket.io/4.8.1/socket.io.min.js"></script>
            <script>
                import { Excalidraw } from "https://cdn.jsdelivr.net/npm/@excalidraw/excalidraw@0.18.0/dist/prod/index.min.js";

                const socket = io("/board");

                document.addEventListener("DOMContentLoaded", () => {
                    const container = document.querySelector(".container");

                    socket.on("change", (data) => {
                        handleChange(data, false);
                    });

                    ReactDOM.createRoot(container).render(
                        React.createElement(Excalidraw, {
                            initialData: {},
                            onChange: (elements, state, files) => {
                                handleChange({ elements, files }, true);
                            },
                        })
                    );
                });

                const handleChange = ({ elements, files }, local = false) => {
                    if (!local) {
                        const updatedElements = new Map(elements.map((e) => [e.id, e]));
                        let sceneElements = state.current.elements;
                        state.current.elements = [
                            ...sceneElements.filter((e) => !updatedElements.has(e.id)).map((e) => ({ ...e })),
                            ...updatedElements.values(),
                        ];

                        hash.current = hashElementsVersion(state.current.elements);
                        if (api.current) {
                            api.current.updateScene({ elements: state.current.elements, captureUpdate: CaptureUpdateAction.NEVER });
                            api.current.addFiles(Object.values(files));
                        }
                    } else {
                        const currentHash = hashElementsVersion(elements);
                        if (currentHash !== hash.current) {
                            hash.current = currentHash;

                            const previousElements = new Map((state.current.elements || []).map((el) => [el.id, el]));
                            const updatedElements = elements.filter((e) => {
                                const prev = previousElements.get(e.id);
                                return !prev || prev.version !== e.version;
                            });
                            if (updatedElements.length == 0) return;

                            state.current = { elements: elements.map((e) => ({ ...e })), files: { ...files } };
                            socket.emit("change", { elements: updatedElements, files: {} });
                        }
                    }
                };
            </script>
        </body>
        </html>
        "#,
    )
}
