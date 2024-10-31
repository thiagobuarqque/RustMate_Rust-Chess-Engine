export default class Confetti {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private particles: Particle[] = [];
    private width: number;
    private height: number;
    private stopped = false;

    constructor() {
        this.canvas = document.createElement('canvas');
        this.ctx = this.canvas.getContext('2d') as CanvasRenderingContext2D;
        this.width = window.innerWidth;
        this.height = window.innerHeight;

        this.canvas.style.position = 'fixed';
        this.canvas.style.top = '0';
        this.canvas.style.left = '0';
        this.canvas.style.width = `${this.width}px`;
        this.canvas.style.height = `${this.height}px`;
        this.canvas.style.pointerEvents = 'none';
        this.canvas.style.zIndex = '9999';

        document.body.appendChild(this.canvas);

        this.resizeCanvas();
        window.addEventListener('resize', this.resizeCanvas);

        this.initParticles();

        setTimeout(() => {
            this.canvas.style.transition = 'all ease-in-out 2s'
            this.canvas.classList.add('opacity-0');
        }, 10000);

        setTimeout(() => {
            this.stopped = true;
            document.body.removeChild(this.canvas); // Remove the canvas when done
        }, 14000);

        requestAnimationFrame(() => this.update());
    }

    private resizeCanvas = () => {
        this.width = window.innerWidth;
        this.height = window.innerHeight;
        this.canvas.width = this.width;
        this.canvas.height = this.height;
    };

    private initParticles() {
        for (let i = 0; i < 100; i++) {
            this.particles.push(new Particle(this.width, this.height));
        }
    }

    private update = () => {
        if (this.stopped) return;

        this.ctx.clearRect(0, 0, this.width, this.height);
        this.particles.forEach(particle => particle.update(this.ctx));
        requestAnimationFrame(this.update);
    }
}

class Particle {
    private x: number;
    private y: number;
    private width: number;
    private height: number;
    private speedY: number;
    private speedX: number;
    private color: string;
    private rotation: number;
    private rotationSpeed: number;

    constructor(canvasWidth: number, canvasHeight: number) {
        this.x = Math.random() * canvasWidth;
        this.y = Math.random() * canvasHeight;
        this.width = Math.random() * 5 + 2;
        this.height = Math.random() * 8 + 3;
        this.speedY = Math.random() * 2 + 1;
        this.speedX = (Math.random() - 0.5) * 2;
        this.color = `hsl(${Math.random() * 360}, 100%, 50%)`;

        // Initialize rotation properties
        this.rotation = Math.random() * 360; // Start at a random rotation angle
        this.rotationSpeed = Math.random() * 5 + 1; // Speed of rotation
    }

    update(ctx: CanvasRenderingContext2D) {
        this.y += this.speedY;
        this.x += this.speedX;
        this.rotation += this.rotationSpeed; // Update rotation angle

        // Reset particle if it goes out of bounds
        if (this.y > ctx.canvas.height) {
            this.y = 0;
            this.x = Math.random() * ctx.canvas.width;
        }

        ctx.save();
        ctx.translate(this.x, this.y);
        ctx.rotate((this.rotation * Math.PI) / 180);

        // Scale to simulate flipping in 3D
        const scaleX = Math.abs(Math.cos((this.rotation * Math.PI) / 180));
        ctx.scale(scaleX, 1); // Only scale on the x-axis for the flip effect

        ctx.fillStyle = this.color;
        ctx.fillRect(-this.width / 2, -this.height / 2, this.width, this.height);

        ctx.restore();
    }
}
