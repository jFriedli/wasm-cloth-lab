import init, { Simulation } from "./wasm/cloth_wasm.js";
import { FRESH_DEFAULTS, qualityMesh, QualityPreset } from "./defaults";
import { MobileDeviceMotionSource } from "./mobile-motion";
import { DesktopWindowMotionSource, mobileCanonicalMotion } from "./motion-source";
import { mapMotion } from "./motion";
import { Renderer } from "./renderer";
import "./style.css";

const el = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const canvas = el<HTMLCanvasElement>("scene"), panel = el("panel"), metrics = el<HTMLPreElement>("metrics"), error = el("error");
const settings = el<HTMLButtonElement>("settings"), material = el<HTMLSelectElement>("material"), quality = el<HTMLSelectElement>("quality"), attachment = el<HTMLSelectElement>("attachment"), wind = el<HTMLInputElement>("wind"), motion = el<HTMLInputElement>("motion"), inertia = el<HTMLInputElement>("inertia");
const enableMotion = el<HTMLButtonElement>("enable-motion"), motionStatus = el("motion-status"), hintTitle = el("hint-title"), hintDetail = el("hint-detail");
quality.value = String(FRESH_DEFAULTS.quality); attachment.value = String(FRESH_DEFAULTS.attachment);
settings.onclick = () => { const open = panel.classList.toggle("open"); settings.setAttribute("aria-expanded", String(open)); };

const format3 = (v: { x: number; y: number; z: number }) => `${v.x.toFixed(2)}, ${v.y.toFixed(2)}, ${v.z.toFixed(2)}`;

async function boot() {
  const wasm = await init(), renderer = new Renderer(canvas), desktop = new DesktopWindowMotionSource(), mobile = new MobileDeviceMotionSource();
  mobile.startPassive();
  if (mobile.permission === "needs-permission") {
    enableMotion.hidden = false; motionStatus.hidden = false;
    motionStatus.textContent = "Enable motion to move the cloth with your phone.";
    hintTitle.textContent = "Enable motion to move the cloth."; hintDetail.textContent = "Tap TUNE, then Enable phone motion.";
  }
  enableMotion.onclick = async () => {
    const permission = await mobile.requestPermission(); motionStatus.hidden = false;
    if (permission === "granted") {
      enableMotion.hidden = true; motionStatus.textContent = "Phone motion enabled. Move or shake your phone.";
      hintTitle.textContent = "Move or shake your phone."; hintDetail.textContent = "Physical motion becomes wind and inertia.";
    } else motionStatus.textContent = "Motion permission was not granted. Touch dragging still works.";
  };

  let sim: Simulation, q = qualityMesh(FRESH_DEFAULTS.quality), paused = false, debug = false, gust = 0, last = performance.now(), acc = 0, frames = 0, fps = 0, fpsAt = last, physicsMs = 0;
  const create = () => { q = qualityMesh(Number(quality.value) as QualityPreset); sim = new Simulation(q.w, q.h, Number(material.value)); sim.set_attachment(Number(attachment.value)); const mem = wasm.memory.buffer; renderer.setMesh(new Float32Array(mem, sim.positions_ptr(), sim.vertex_count() * 3), new Float32Array(mem, sim.normals_ptr(), sim.vertex_count() * 3), new Uint32Array(mem, sim.indices_ptr(), sim.index_count()), q.w, q.h); };
  create();
  const worldAt = (e: PointerEvent) => { const halfY = 2.7, aspect = canvas.clientWidth / canvas.clientHeight; return { x: ((e.clientX / canvas.clientWidth) * 2 - 1) * halfY * aspect + 1.25, y: (1 - (e.clientY / canvas.clientHeight) * 2) * halfY }; };
  let grabbing = false;
  canvas.addEventListener("pointerdown", e => { const p = worldAt(e); if (p.x < -.15 || p.x > 2.9 || p.y < -1 || p.y > 1) return; grabbing = true; canvas.setPointerCapture(e.pointerId); sim.set_grab(p.x / 2.7, (.825 - p.y) / 1.65, p.x, p.y, .35); });
  canvas.addEventListener("pointermove", e => { if (grabbing) { const p = worldAt(e); sim.move_grab(p.x, p.y, .35); } });
  const release = () => { if (grabbing) sim.clear_grab(); grabbing = false; }; canvas.addEventListener("pointerup", release); canvas.addEventListener("pointercancel", release);
  const reset = () => create(); el("reset").onclick = reset; quality.onchange = reset; material.onchange = reset; attachment.onchange = () => sim.set_attachment(Number(attachment.value));
  el("gust").onclick = () => gust = 8; el("debug").onclick = () => { debug = !debug; metrics.hidden = !debug; };
  wind.oninput = () => el<HTMLOutputElement>("wind-out").value = Number(wind.value).toFixed(1); motion.oninput = () => el<HTMLOutputElement>("motion-out").value = Number(motion.value).toFixed(2); inertia.oninput = () => el<HTMLOutputElement>("inertia-out").value = Number(inertia.value).toFixed(2);
  el<HTMLInputElement>("image").onchange = async e => { try { const file = (e.currentTarget as HTMLInputElement).files?.[0]; if (!file) return; if (!file.type.startsWith("image/") || file.size > 20_000_000) throw new Error("Choose a valid image smaller than 20 MB"); const bitmap = await createImageBitmap(file); renderer.setImage(bitmap); bitmap.close(); } catch (reason) { error.hidden = false; error.textContent = reason instanceof Error ? reason.message : String(reason); } };
  addEventListener("keydown", e => { if (e.ctrlKey || e.metaKey || e.altKey || /INPUT|SELECT/.test((e.target as Element).tagName)) return; if (e.code === "KeyR") reset(); if (e.code === "KeyF") gust = 8; if (e.code === "KeyD") { debug = !debug; metrics.hidden = !debug; } if (e.code === "Space") { e.preventDefault(); paused = !paused; } if (/^Digit[1-4]$/.test(e.code)) { quality.value = String(Number(e.code.at(-1)) - 1); reset(); } });
  document.addEventListener("visibilitychange", () => { desktop.reset({ x: screenX, y: screenY }, performance.now()); mobile.reset(); last = performance.now(); acc = 0; });
  let announcedMobile = false;
  const loop = (now: number) => {
    const frame = Math.min(.1, (now - last) / 1000); last = now; acc = Math.min(.05, acc + frame);
    const windScale = Number(motion.value), inertiaScale = Number(inertia.value), mobileState = mobile.state(now);
    const canonical = mobile.active ? mobileCanonicalMotion(mobileState, windScale, inertiaScale) : desktop.sample({ x: screenX, y: screenY }, now, windScale, inertiaScale);
    if (mobile.active && !announcedMobile) {
      announcedMobile = true; hintTitle.textContent = "Move or shake your phone."; hintDetail.textContent = "Physical motion becomes wind and inertia.";
      motionStatus.hidden = false; motionStatus.textContent = "Phone motion active.";
      el("motion-label").textContent = "Motion wind"; el("inertia-label").textContent = "Motion inertia";
    }
    const ambient = Number(wind.value), turbulence = Math.sin(now * .0017) + .45 * Math.sin(now * .0031 + 2.1); gust *= Math.exp(-frame * 1.7);
    const airflow = { x: canonical.relativeWind.x + ambient + gust, y: canonical.relativeWind.y, z: 1.1 + turbulence * .55 + gust * .25 + canonical.relativeWind.z };
    const before = performance.now(); let steps = 0;
    if (!paused) while (acc >= 1 / 120 && steps < 6) { sim.step(1 / 120, 9.81, airflow.x, airflow.y, airflow.z, canonical.inertia.x, canonical.inertia.y, canonical.inertia.z); acc -= 1 / 120; steps++; }
    physicsMs = physicsMs * .9 + (performance.now() - before) * .1;
    const mem = wasm.memory.buffer; renderer.update(new Float32Array(mem, sim.positions_ptr(), sim.vertex_count() * 3), new Float32Array(mem, sim.normals_ptr(), sim.vertex_count() * 3)); renderer.render();
    frames++; if (now - fpsAt > 500) { fps = frames * 1000 / (now - fpsAt); frames = 0; fpsAt = now; }
    if (debug) {
      const d = desktop.last, mappedVelocity = mapMotion(d.velocity, 12, 2400), mappedAcceleration = mapMotion(d.acceleration, 120, 8500);
      metrics.textContent = `FPS             ${fps.toFixed(1)}\nframe           ${(frame * 1000).toFixed(2)} ms\nphysics         ${physicsMs.toFixed(2)} ms\nvertices        ${sim.vertex_count()}\ntriangles       ${sim.index_count() / 3}\nconstraints     ${sim.constraint_count()}\nmaterial        ${material.options[material.selectedIndex].text}\nmotion source   ${canonical.source}\npermission      ${mobile.permission}\nraw position    ${d.rawPosition.x}, ${d.rawPosition.y}\nraw vel px/s    ${d.rawVelocity.x.toFixed(1)}, ${d.rawVelocity.y.toFixed(1)}\nfiltered vel    ${d.velocity.x.toFixed(1)}, ${d.velocity.y.toFixed(1)}\nmapped vel      ${mappedVelocity.x.toFixed(1)}, ${mappedVelocity.y.toFixed(1)}\nraw accel       ${d.rawAcceleration.x.toFixed(1)}, ${d.rawAcceleration.y.toFixed(1)}\nfiltered accel  ${d.acceleration.x.toFixed(1)}, ${d.acceleration.y.toFixed(1)}\nmapped accel    ${mappedAcceleration.x.toFixed(1)}, ${mappedAcceleration.y.toFixed(1)}\nsensor raw      ${format3(mobileState.raw)}\nsensor +gravity ${format3(mobileState.includingGravity)}\nsensor filtered ${format3(mobileState.filtered)}\nviewport accel  ${format3(mobileState.viewportAcceleration)}\ngesture velocity ${format3(mobileState.gestureVelocity)}\nsensor source   ${mobileState.source}\nsensor rate     ${mobileState.frequency.toFixed(1)} Hz\norientation     ${mobileState.orientation}°\nstationary      ${mobileState.stationary} (${mobileState.stale ? "stale" : "live"})\nwind gain       ${canonical.source === "mobile-device" ? "6.0" : ".016"} × ${windScale.toFixed(2)}\ninertia gain    ${canonical.source === "mobile-device" ? "1.6" : ".005"} × ${inertiaScale.toFixed(2)}\nrelative wind   ${format3(canonical.relativeWind)}\ninertia         ${format3(canonical.inertia)}\nairflow         ${format3(airflow)}\nviewport        ${innerWidth} × ${innerHeight} @ ${devicePixelRatio.toFixed(2)}`;
    }
    requestAnimationFrame(loop);
  }; requestAnimationFrame(loop);
}

boot().catch(reason => { console.error(reason); error.hidden = false; error.textContent = reason instanceof Error ? reason.message : String(reason); });
