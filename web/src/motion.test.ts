import {describe,expect,it} from "vitest";import {MotionEstimator,inertiaFromAcceleration,relativeWind}from"./motion";
describe("window motion",()=>{
it("derives velocity and acceleration",()=>{const e=new MotionEstimator({smoothing:1});e.sample({x:0,y:0},0);const s=e.sample({x:10,y:0},100);expect(s.velocity.x).toBeCloseTo(100);expect(s.acceleration.x).toBeGreaterThan(0)});
it("maps motion right to wind left",()=>{expect(relativeWind({x:0,y:0},{x:10,y:0}).x).toBe(-10);expect(inertiaFromAcceleration({x:5,y:0}).x).toBe(-5)});
it("rejects jumps",()=>{const e=new MotionEstimator({smoothing:1,outlierDistance:20});e.sample({x:0,y:0},0);expect(e.sample({x:200,y:0},16).stale).toBe(true)});
it("decays stale velocity",()=>{const e=new MotionEstimator({smoothing:1,staleAfter:100});e.sample({x:0,y:0},0);e.sample({x:10,y:0},50);expect(e.sample({x:10,y:0},250).velocity.x).toBeLessThan(200)});
it("resets after a pause",()=>{const e=new MotionEstimator({smoothing:1,resetAfter:500});e.sample({x:0,y:0},0);const s=e.sample({x:100,y:0},1000);expect(s.velocity.x).toBe(0)});});
