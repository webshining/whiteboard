import { CaptureUpdateAction, Excalidraw, hashElementsVersion } from "@excalidraw/excalidraw";
import type { ExcalidrawElement } from "@excalidraw/excalidraw/element/types";
import "@excalidraw/excalidraw/index.css";
import type { BinaryFiles, ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import { Device } from "mediasoup-client";
import type { RtpCapabilities, TransportOptions } from "mediasoup-client/types";
import { useEffect, useRef } from "react";
import { io, type Socket } from "socket.io-client";
import "./App.css";

function App() {
	const socket = useRef<Socket | null>(null);

	const state = useRef<{ elements: ExcalidrawElement[]; files: BinaryFiles }>({ elements: [], files: {} });
	const hash = useRef<number>(0);
	const api = useRef<ExcalidrawImperativeAPI | null>(null);

	const device = useRef<Device | null>(null);

	useEffect(() => {
		socket.current = io("ws://localhost:4000/");
		device.current = new Device();

		socket.current.emit("rtpCapabilities", {}, async (routerRtpCapabilities: RtpCapabilities) => {
			await device.current!.load({ routerRtpCapabilities });
			socket.current!.emit("createTransport", true, async (transportOptions: TransportOptions) => {
				console.log(1);
				const sendTransport = device.current!.createSendTransport(transportOptions);

				sendTransport.on("connect", ({ dtlsParameters }, callback) => {
					console.log(2);
					socket.current!.emit("connectTransport", dtlsParameters, () => callback());
				});

				sendTransport.on("produce", async ({ kind, rtpParameters }, callback) => {
					console.log(3);
					socket.current!.emit("produce", { kind, rtpParameters }, (id: string) => callback({ id }));
				});

				const stream = await navigator.mediaDevices.getUserMedia({
					audio: { noiseSuppression: false, echoCancellation: false, autoGainControl: false },
				});
				const track = stream.getTracks()[0];
				await sendTransport.produce({ track });
			});
		});

		socket.current.on("change", (data) => {
			handleChange(data, false);
		});
	}, []);

	const handleChange = ({ elements, files }: { elements: ExcalidrawElement[]; files: BinaryFiles }, local = false) => {
		if (!local) {
			const updatedElements = new Map(elements.map((e) => [e.id, e]));
			const sceneElements = state.current.elements;
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
				socket.current?.emit("change", { elements: updatedElements, files });
			}
		}
	};

	return (
		<div className="content__wrapper">
			<div className="content">
				<Excalidraw
					excalidrawAPI={(a) => (api.current = a)}
					gridModeEnabled
					onChange={(elements, _, files) => handleChange({ elements: [...elements], files }, true)}
				/>
			</div>
		</div>
	);
}

export default App;
