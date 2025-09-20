import { CaptureUpdateAction, Excalidraw } from "@excalidraw/excalidraw";
import type { ExcalidrawElement } from "@excalidraw/excalidraw/element/types";
import "@excalidraw/excalidraw/index.css";
import type { BinaryFiles, ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import { useEffect, useRef } from "react";
import { io, type Socket } from "socket.io-client";
import "./App.css";

function App() {
	const socket = useRef<Socket | null>(null);

	const api = useRef<ExcalidrawImperativeAPI | null>(null);
	let versions = useRef<Map<String, Number>>(new Map());

	useEffect(() => {
		socket.current = io("ws://127.0.0.1:4002/");

		socket.current.emit("boardState", (data: { elements: ExcalidrawElement[]; files: BinaryFiles }) => {
			handleChange(data, false);
		});

		socket.current.on("change", (data) => {
			handleChange(data, false);
		});

		return () => {
			socket.current!.close();
			socket.current = null;
		};
	}, []);

	const handleChange = ({ elements, files }: { elements: ExcalidrawElement[]; files: BinaryFiles }, local = false) => {
		elements = elements.filter((e) => {
			const prevVersion = versions.current.get(e.id);
			return !prevVersion || prevVersion !== e.version;
		});
		if (elements.length === 0) return;
		elements.forEach((e) => versions.current.set(e.id, e.version));
		if (local) {
			socket.current!.emit("change", { elements: elements, files: files });
		} else if (!local) {
			const newIds = new Set(elements.map((e) => e.id));
			const sceneElements = api.current!.getSceneElements();
			const newElements = [...sceneElements.filter((e) => !newIds.has(e.id)), ...elements];

			api.current!.updateScene({ elements: newElements, captureUpdate: CaptureUpdateAction.NEVER });
			api.current!.addFiles(Object.values(files));
		}
	};

	return (
		<div className="content__wrapper">
			<div className="content">
				<Excalidraw
					gridModeEnabled
					excalidrawAPI={(a) => (api.current = a)}
					onChange={(e, _, files) => handleChange({ elements: [...e], files }, true)}
				/>
			</div>
		</div>
	);
}

export default App;
