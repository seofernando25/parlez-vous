import { initTTSAudio, playSmartTTS, setLipSyncNode } from '$lib/tts';
import type { TtsProviderPolicy } from '$lib/media/tts-policy';

const ANIMATIONS: Record<string, string> = {
    idle: '/vrm/idle_loop.vrma',
    shrug: '/vrm/VRMA_03.vrma',
    greet: '/vrm/VRMA_02.vrma',
    peace: '/vrm/VRMA_03.vrma',
    shoot: '/vrm/VRMA_04.vrma',
    spin: '/vrm/VRMA_05.vrma',
    pose: '/vrm/VRMA_06.vrma',
    squat: '/vrm/VRMA_07.vrma',
    full: '/vrm/VRMA_01.vrma'
};

export class AvatarStageController {
    currentVrm = $state<any>(null);
    isLoading = $state(true);

    private THREE: any;
    private GLTFLoader: any;
    private VRMLoaderPlugin: any;
    private VRMAnimationLoaderPlugin: any;
    private createVRMAnimationClip: any;
    private createWLipSyncNode: any;
    private scene: any;
    private camera: any;
    private renderer: any;
    private mixer: any;
    private timer: any;
    private lipSyncNode: any;
    private animations = new Map<string, any>();
    private idleAction: any;
    private activeAction: any;
    private speaking = false;
    private frameId?: number;
    private resizeObserver?: ResizeObserver;
    private container?: HTMLDivElement;

    constructor(
        private getVrm: () => string,
        private getTtsUrl: () => string,
        private getLanguage: () => string,
        private getTtsPolicy: () => TtsProviderPolicy
    ) {}

    async mount(container: HTMLDivElement) {
        if (this.renderer) return;
        this.container = container;
        await this.loadLibraries();
        this.initScene();
        await this.loadVrm(this.getVrm());
    }

    private async loadLibraries() {
        this.THREE = await import('three');
        this.timer = new this.THREE.Timer();
        this.GLTFLoader = (await import('three/examples/jsm/loaders/GLTFLoader.js')).GLTFLoader;
        const vrm = await import('@pixiv/three-vrm');
        this.VRMLoaderPlugin = vrm.VRMLoaderPlugin;
        const anim = await import('@pixiv/three-vrm-animation');
        this.VRMAnimationLoaderPlugin = anim.VRMAnimationLoaderPlugin;
        this.createVRMAnimationClip = anim.createVRMAnimationClip;
        this.createWLipSyncNode = (await import('wlipsync')).createWLipSyncNode;
    }

    private initScene() {
        if (!this.container) return;
        this.scene = new this.THREE.Scene();
        this.scene.background = new this.THREE.Color('#0a0a0a');
        const light = new this.THREE.DirectionalLight(0xffffff, 1.5);
        light.position.set(1, 1, 1).normalize();
        this.scene.add(light, new this.THREE.AmbientLight(0x404040, 1));
        this.camera = new this.THREE.PerspectiveCamera(30, 1, 0.1, 20);
        this.camera.position.set(0, 1.4, 2);
        this.renderer = new this.THREE.WebGLRenderer({ antialias: true, alpha: true });
        this.renderer.domElement.style.cssText = 'width:100%;height:100%;display:block';
        this.container.appendChild(this.renderer.domElement);
        this.resize();
        window.addEventListener('resize', this.resize);
        this.resizeObserver = new ResizeObserver(this.resize);
        this.resizeObserver.observe(this.container);
        this.frameId = requestAnimationFrame(this.animate);
    }

    resize = () => {
        if (!this.container || !this.camera || !this.renderer) return;
        const width = this.container.clientWidth;
        const height = this.container.clientHeight;
        if (width <= 0 || height <= 0) return;
        this.camera.aspect = width / height;
        this.camera.updateProjectionMatrix();
        this.renderer.setPixelRatio(window.devicePixelRatio);
        this.renderer.setSize(width, height, false);
    };

    private animate = (timestamp: number) => {
        this.frameId = requestAnimationFrame(this.animate);
        this.timer.update(timestamp);
        const delta = this.timer.getDelta();
        this.mixer?.update(delta);
        this.updateLipSync();
        this.currentVrm?.update(delta);
        this.renderer?.render(this.scene, this.camera);
    };

    private updateLipSync() {
        if (!this.currentVrm || !this.lipSyncNode || !this.speaking) return;
        const manager = this.currentVrm.expressionManager;
        const weights = this.lipSyncNode.weights;
        if (!manager || !weights) return;
        manager.setValue('aa', weights.A || 0);
        manager.setValue('ih', weights.I || 0);
        manager.setValue('ou', weights.U || 0);
        manager.setValue('ee', weights.E || 0);
        manager.setValue('oh', weights.O || 0);
    }

    private async loadVrm(name: string) {
        this.isLoading = true;
        const loader = new this.GLTFLoader();
        loader.register((parser: any) => new this.VRMLoaderPlugin(parser));
        loader.register((parser: any) => new this.VRMAnimationLoaderPlugin(parser));
        await new Promise<void>(resolve => loader.load(`/vrm/${name}`, (gltf: any) => {
            const vrm = gltf.userData.vrm;
            this.scene.add(vrm.scene);
            this.currentVrm = vrm;
            vrm.scene.rotation.y = Math.PI;
            this.centerVrm(vrm);
            this.mixer = new this.THREE.AnimationMixer(vrm.scene);
            this.preloadAnimations();
            this.isLoading = false;
            resolve();
        }, undefined, (error: ErrorEvent) => {
            console.error(error);
            this.isLoading = false;
            resolve();
        }));
    }

    private centerVrm(vrm: any) {
        const scene = vrm.scene;
        scene.updateMatrixWorld(true);

        // Use precise skinned-geometry bounds: some VRMs have a centered humanoid
        // rig but meshes that are offset from it, so bone anchors are not a visual center.
        const bounds = new this.THREE.Box3().setFromObject(scene, true);
        if (bounds.isEmpty()) return;
        const center = bounds.getCenter(new this.THREE.Vector3());
        scene.position.x -= center.x;
        // Perspective and asymmetric clothing make the silhouette read left of
        // its geometric center. A small optical offset keeps the avatar centered.
        scene.position.x += 0.15;
        scene.updateMatrixWorld(true);
    }

    private preloadAnimations() {
        if (!this.currentVrm || !this.mixer) return;
        const loader = new this.GLTFLoader();
        loader.register((parser: any) => new this.VRMAnimationLoaderPlugin(parser));
        for (const [code, file] of Object.entries(ANIMATIONS)) loader.load(file, (gltf: any) => {
            const source = gltf.userData.vrmAnimations?.[0];
            if (!source) return;
            const clip = this.createVRMAnimationClip(source, this.currentVrm);
            this.animations.set(code, clip);
            if (code === 'idle') {
                this.idleAction = this.mixer.clipAction(clip);
                this.idleAction.play();
            }
        });
    }

    playAnimation(code: string) {
        if (!this.currentVrm || !this.mixer) return;
        const clip = this.animations.get(this.animations.has(code.toLowerCase()) ? code.toLowerCase() : 'pose');
        if (!clip) return;
        this.activeAction?.fadeOut(0.2);
        this.activeAction = this.mixer.clipAction(clip);
        this.activeAction.setLoop(this.THREE.LoopOnce, 1);
        this.activeAction.clampWhenFinished = true;
        if (this.idleAction) this.activeAction.crossFadeFrom(this.idleAction, 0.2, false);
        this.activeAction.reset().fadeIn(0.2).play();
        const onFinished = (event: any) => {
            if (event.action !== this.activeAction) return;
            this.mixer.removeEventListener('finished', onFinished);
            this.idleAction?.reset().fadeIn(0.2).play();
            this.activeAction?.fadeOut(0.2);
            this.activeAction = null;
        };
        this.mixer.addEventListener('finished', onFinished);
    }

    async speak(text: string, muted: boolean) {
        if (muted) return;
        await this.initLipSync();
        this.speaking = true;
        try {
            await playSmartTTS(text, this.getTtsUrl(), code => this.playAnimation(code), this.getLanguage(), this.getTtsPolicy());
        } finally {
            this.speaking = false;
            this.resetMouth();
        }
    }

    private async initLipSync() {
        initTTSAudio();
        if (this.lipSyncNode) return;
        try {
            const response = await fetch('/profile.json');
            if (!response.ok) throw new Error('profile.json not found');
            const profile = await response.json();
            if (!Array.isArray(profile.mfccs) || !profile.mfccs.length) throw new Error('Invalid wlipsync profile');
            const { ttsAudioContext } = await import('$lib/tts');
            this.lipSyncNode = await this.createWLipSyncNode(ttsAudioContext, profile);
            setLipSyncNode(this.lipSyncNode);
        } catch (error) {
            console.warn('Lip sync unavailable:', error);
        }
    }

    private resetMouth() {
        const manager = this.currentVrm?.expressionManager;
        if (manager) ['aa', 'ih', 'ou', 'ee', 'oh'].forEach(vowel => manager.setValue(vowel, 0));
    }

    dispose() {
        if (this.frameId) cancelAnimationFrame(this.frameId);
        this.resizeObserver?.disconnect();
        window.removeEventListener('resize', this.resize);
        this.renderer?.dispose();
        this.renderer = null;
        this.container = undefined;
    }
}
