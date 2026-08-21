import init, { Simulation } from "./wasm/cloth_wasm.js";
import { MotionEstimator, relativeWind } from "./motion";
import { Renderer } from "./renderer";
import "./style.css";

const qualities = [{ w: 30, h: 20 }, { w: 50, h: 32 }, { w: 75, h: 48 }, { w: 100, h: 64 }];
const el = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const canvas = el<HTMLCanvasElement>("scene"), panel = el("panel"), metrics = el<HTMLPreElement>("metrics"), error = el("error");
const settings = el<HTMLButtonElement>("settings"), material = el<HTMLSelectElement>("material"), quality = el<HTMLSelectElement>("quality"), attachment = el<HTMLSelectElement>("attachment"), wind = el<HTMLInputElement>("wind"), motion = el<HTMLInputElement>("motion");
settings.onclick = () => { const open = panel.classList.toggle("open"); settings.setAttribute("aria-expanded", String(open)); };

async function boot() {
  const wasm = await init(); const renderer = new Renderer(canvas); const estimator = new MotionEstimator();
  let sim: Simulation, q = qualities[1], paused = false, debug = false, gust = 0, last = performance.now(), acc = 0, frames = 0, fps = 0, fpsAt = last, physicsMs = 0;
  const create = () => { q = qualities[Number(quality.value)]; sim = new Simulation(q.w, q.h, Number(material.value)); sim.set_attachment(Number(attachment.value)); const mem = wasm.memory.buffer; renderer.setMesh(new Float32Array(mem, sim.positions_ptr(), sim.vertex_count() * 3), new Float32Array(mem, sim.normals_ptr(), sim.vertex_count() * 3), new Uint32Array(mem, sim.indices_ptr(), sim.index_count()), q.w, q.h); };
  create();
  const worldAt = (e: PointerEvent) => { const halfY = 2.7, aspect = canvas.clientWidth / canvas.clientHeight; return { x: ((e.clientX / canvas.clientWidth) * 2 - 1) * halfY * aspect + 1.25, y: (1 - (e.clientY / canvas.clientHeight) * 2) * halfY }; };
  let grabbing = false;
  canvas.addEventListener("pointerdown", e => { const p = worldAt(e); if (p.x < -.15 || p.x > 2.9 || p.y < -1 || p.y > 1) return; grabbing = true; canvas.setPointerCapture(e.pointerId); sim.set_grab(p.x / 2.7, (.825 - p.y) / 1.65, p.x, p.y, .35); });
  canvas.addEventListener("pointermove", e => { if (grabbing) { const p = worldAt(e); sim.move_grab(p.x, p.y, .35); } });
  const release = () => { if (grabbing) sim.clear_grab(); grabbing = false; }; canvas.addEventListener("pointerup", release); canvas.addEventListener("pointercancel", release);
  const reset = () => create(); el("reset").onclick = reset; quality.onchange = reset; material.onchange = reset; attachment.onchange = () => sim.set_attachment(Number(attachment.value));
  el("gust").onclick = () => gust = 8; el("debug").onclick = () => { debug = !debug; metrics.hidden = !debug; };
  wind.oninput = () => el<HTMLOutputElement>("wind-out").value = Number(wind.value).toFixed(1); motion.oninput = () => el<HTMLOutputElement>("motion-out").value = Number(motion.value).toFixed(2);
  el<HTMLInputElement>("image").onchange = async e => { try { const file = (e.currentTarget as HTMLInputElement).files?.[0]; if (!file) return; if (!file.type.startsWith("image/") || file.size > 20_000_000) throw new Error("Choose a valid image smaller than 20 MB"); const bitmap = await createImageBitmap(file); renderer.setImage(bitmap); bitmap.close(); } catch (reason) { error.hidden = false; error.textContent = reason instanceof Error ? reason.message : String(reason); } };
  addEventListener("keydown", e => { if (e.ctrlKey || e.metaKey || e.altKey || /INPUT|SELECT/.test((e.target as Element).tagName)) return; if (e.code === "KeyR") reset(); if (e.code === "KeyF") gust = 8; if (e.code === "KeyD") { debug = !debug; metrics.hidden = !debug; } if (e.code === "Space") { e.preventDefault(); paused = !paused; } if (/^Digit[1-4]$/.test(e.code)) { quality.value = String(Number(e.code.at(-1)) - 1); reset(); } });
  document.addEventListener("visibilitychange", () => { estimator.reset({ x: screenX, y: screenY }, performance.now()); last = performance.now(); acc = 0; });
  const loop = (now: number) => {
    const frame = Math.min(.1, (now - last) / 1000); last = now; acc = Math.min(.05, acc + frame);
    const m = estimator.sample({ x: screenX, y: screenY }, now), scale = Number(motion.value);
    const rw = relativeWind({ x: 0, y: 0 }, { x: m.velocity.x * .006 * scale, y: -m.velocity.y * .006 * scale });
    const ambient = Number(wind.value), turbulence = Math.sin(now * .0017) + .45 * Math.sin(now * .0031 + 2.1); gust *= Math.exp(-frame * 1.7);
    const airflow = { x: rw.x + ambient + gust, y: rw.y * .35, z: 1.1 + turbulence * .55 + gust * .25 };
    const ax = m.acceleration.x * .0018 * scale, ay = -m.acceleration.y * .0018 * scale, before = performance.now(); let steps = 0;
    if (!paused) while (acc >= 1 / 120 && steps < 6) { sim.step(1 / 120, 9.81, airflow.x, airflow.y, airflow.z, ax, ay, 0); acc -= 1 / 120; steps++; }
    physicsMs = physicsMs * .9 + (performance.now() - before) * .1;
    const mem = wasm.memory.buffer; renderer.update(new Float32Array(mem, sim.positions_ptr(), sim.vertex_count() * 3), new Float32Array(mem, sim.normals_ptr(), sim.vertex_count() * 3)); renderer.render();
    frames++; if (now - fpsAt > 500) { fps = frames * 1000 / (now - fpsAt); frames = 0; fpsAt = now; }
    if (debug) metrics.textContent = `FPS             ${fps.toFixed(1)}\nframe           ${(frame * 1000).toFixed(2)} ms\nphysics         ${physicsMs.toFixed(2)} ms\nvertices        ${sim.vertex_count()}\ntriangles       ${sim.index_count() / 3}\nconstraints     ${sim.constraint_count()}\nmaterial        ${material.options[material.selectedIndex].text}\nraw position    ${m.rawPosition.x}, ${m.rawPosition.y}\nvelocity px/s   ${m.velocity.x.toFixed(1)}, ${m.velocity.y.toFixed(1)}\naccel px/s²     ${m.acceleration.x.toFixed(1)}, ${m.acceleration.y.toFixed(1)}\nrelative wind   ${rw.x.toFixed(2)}, ${rw.y.toFixed(2)}\ninertia         ${(-ax).toFixed(2)}, ${(-ay).toFixed(2)}\nairflow         ${airflow.x.toFixed(2)}, ${airflow.y.toFixed(2)}, ${airflow.z.toFixed(2)}\nviewport        ${innerWidth} × ${innerHeight} @ ${devicePixelRatio.toFixed(2)}`;
    requestAnimationFrame(loop);
  }; requestAnimationFrame(loop);
}
boot().catch(reason => { console.error(reason); error.hidden = false; error.textContent = reason instanceof Error ? reason.message : String(reason); });
