export type V2={x:number;y:number};
export interface MotionState { rawPosition:V2; velocity:V2; acceleration:V2; stale:boolean }
export interface MotionConfig { smoothing:number; maxVelocity:number; maxAcceleration:number; outlierDistance:number; staleAfter:number; resetAfter:number }
const zero=():V2=>({x:0,y:0}); const clamp=(v:V2,max:number)=>{const l=Math.hypot(v.x,v.y);return l>max?{x:v.x*max/l,y:v.y*max/l}:v};
export class MotionEstimator {
  readonly config:MotionConfig; private last?:{p:V2;t:number}; private v=zero();private a=zero();
  constructor(config:Partial<MotionConfig>={}){this.config={smoothing:.22,maxVelocity:2600,maxAcceleration:9000,outlierDistance:900,staleAfter:160,resetAfter:1000,...config}}
  reset(p:V2={x:0,y:0},t=0):MotionState{this.last={p,t};this.v=zero();this.a=zero();return{rawPosition:p,velocity:this.v,acceleration:this.a,stale:false}}
  sample(p:V2,t:number):MotionState {
    if(!this.last)return this.reset(p,t);const dt=(t-this.last.t)/1000;
    if(dt<=0||dt>this.config.resetAfter/1000)return this.reset(p,t);
    if(Math.hypot(p.x-this.last.p.x,p.y-this.last.p.y)>this.config.outlierDistance){this.last={p,t};return{rawPosition:p,velocity:{...this.v},acceleration:{...this.a},stale:true}}
    const stale=dt>this.config.staleAfter/1000;const raw=stale?zero():{x:(p.x-this.last.p.x)/dt,y:(p.y-this.last.p.y)/dt};const alpha=stale?Math.min(1,dt*5):1-Math.pow(1-this.config.smoothing,dt*60);
    const old={...this.v};this.v=clamp({x:old.x+(raw.x-old.x)*alpha,y:old.y+(raw.y-old.y)*alpha},this.config.maxVelocity);
    const rawA=stale?zero():{x:(this.v.x-old.x)/dt,y:(this.v.y-old.y)/dt};this.a=clamp({x:this.a.x+(rawA.x-this.a.x)*alpha,y:this.a.y+(rawA.y-this.a.y)*alpha},this.config.maxAcceleration);this.last={p,t};return{rawPosition:p,velocity:{...this.v},acceleration:{...this.a},stale};
  }
}
export const relativeWind=(world:V2,containerVelocity:V2):V2=>({x:world.x-containerVelocity.x,y:world.y-containerVelocity.y});
export const inertiaFromAcceleration=(a:V2):V2=>({x:-a.x,y:-a.y});
