#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D img_output;

////////////////////////////////////////////////////////////////////////////////
// Uniform variables
////////////////////////////////////////////////////////////////////////////////
//TODO: Camera
vec3 sunDir = normalize(vec3(-0.3,1.3,0.1)); //TODO: Uniform
vec3 sunCol =  6.0*vec3(1.0,0.8,0.6); //TODO: Uniform, separate intensity
vec3 skyCol =  4.0*vec3(0.2,0.35,0.5); //TODO: Uniform, separate intensity

// uniform int iSample;
uniform float iSeed;
int iSample = 1;
// float iSeed = 0.47;

////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////
#define EPSILON 0.0001 //[nomod]
#define PI 3.14159265359 //[nomod]
#define TWO_PI 6.28318530718 //[nomod]
#define BOUNCES 4
#define RAYSTEPS 128
#define SHADOWSTEPS 64

////////////////////////////////////////////////////////////////////////////////
// Map
////////////////////////////////////////////////////////////////////////////////
float maxcomp(in vec3 p ) { return max(p.x,max(p.y,p.z));}

float sdBox( vec3 p, vec3 b )
{
  vec3  di = abs(p) - b;
  float mc = maxcomp(di);
  return min(mc,length(max(di,0.0)));
}

float sdSphere( vec3 pos, float sph ) {
    return length(pos) - sph;
}

vec2 map( vec3 p )
{
    vec3 w = p;
    vec3 q = p;

    q.xz = mod( q.xz+1.0, 2.0 ) -1.0;

    float d = sdBox(q,vec3(1.0));
    float s = 1.0;
    for( int m=0; m<6; m++ )
    {
        float h = float(m)/6.0;

        p =  q - 0.5*sin( abs(p.y) + float(m)*3.0+vec3(0.0,3.0,1.0));

        vec3 a = mod( p*s, 2.0 )-1.0;
        s *= 3.0;
        vec3 r = abs(1.0 - 3.0*abs(a));

        float da = max(r.x,r.y);
        float db = max(r.y,r.z);
        float dc = max(r.z,r.x);
        float c = (min(da,min(db,dc))-1.0)/s;

        d = max( c, d );
   }

   float d1 = length(w-vec3(0.22,0.35,0.4)) - 0.09;
   d = min( d, d1 );

   float d2 = w.y + 0.22;
   d =  min( d,d2);

   return vec2(d, 3.0);
}

vec3 calcColour(float mat)
{
    vec3 col = vec3(0.0);

    if (mat < 3.5) col = vec3(242.0 / 255.0, 233.0 / 255.0, 206.0 / 255.0);
    if (mat < 2.5) col = vec3(1.0, 0.0, 0.0);
    if (mat < 1.5) col = vec3(0.0, 1.0, 0.0);
    if (mat < 0.5) col = vec3(1.0, 1.0, 1.0);

    return col;
}

////////////////////////////////////////////////////////////////////////////////
// Hash
////////////////////////////////////////////////////////////////////////////////
float hash(inout float seed)
{
    return fract(sin(seed+=0.1)*43758.5453123 );
}

vec2 hash2(inout float seed) {
    return fract(sin(vec2(seed+=0.1,seed+=0.1))*vec2(43758.5453123,22578.1459123));
}

vec3 cosineDirection( inout float seed, in vec3 nor)
{
    seed += 78.233 * iSeed;
    float u = hash( seed );
    seed += 10.873 * iSeed;
    float v = hash( seed );

	// method by fizzer: http://www.amietia.com/lambertnotangent.html
    float a = 6.2831853 * v;
    u = 2.0*u - 1.0;
    return normalize( nor + vec3(sqrt(1.0-u*u) * vec2(cos(a), sin(a)), u) );
}

vec3 cosWeightedRandomHemisphereDirection( inout float seed, in vec3 n ) {
  	vec2 rv2 = hash2(seed);

	vec3  uu = normalize( cross( n, vec3(0.0,1.0,1.0) ) );
	vec3  vv = normalize( cross( uu, n ) );

	float ra = sqrt(rv2.y);
	float rx = ra*cos(6.2831*rv2.x);
	float ry = ra*sin(6.2831*rv2.x);
	float rz = sqrt( 1.0-rv2.y );
	vec3  rr = vec3( rx*uu + ry*vv + rz*n );

    return normalize( rr );
}

////////////////////////////////////////////////////////////////////////////////
// Lighting
////////////////////////////////////////////////////////////////////////////////
vec3 calcNormal( in vec3 pos )
{
    vec3 eps = vec3(0.0001,0.0,0.0);

    return normalize( vec3(
      map( pos+eps.xyy ).x - map( pos-eps.xyy ).x,
      map( pos+eps.yxy ).x - map( pos-eps.yxy ).x,
      map( pos+eps.yyx ).x - map( pos-eps.yyx ).x ) );
}

//TODO: Define constants for tmax (and possibly precision)
vec2 intersect( in vec3 ro, in vec3 rd )
{
    vec2 res = vec2(-1.0);
    float tmax = 16.0;
    float t = 0.01;
    float m = -1.0;
    for(int i=0; i<RAYSTEPS; i++)
    {
        vec2 tres = map(ro+rd*t);
        float h = tres.x;
        if( h<0.0001 || t>tmax ) break;
        t +=  h;
        m = tres.y;
    }

    if( t<tmax ) res = vec2(t, m);

    return res;
}

//TODO: Colour
float shadow( in vec3 ro, in vec3 rd )
{
    float res = 0.0;

    float tmax = 12.0;

    float t = 0.001;
    for(int i=0; i<SHADOWSTEPS; i++ )
    {
        float h = map(ro+rd*t).x;
        if( h<0.0001 || t>tmax) break;
        t += h;
    }

    if( t>tmax ) res = 1.0;

    return res;
}

vec3 calculateLight(vec3 ro, vec3 rd, inout float sa)
{
    // vec3 albedo = vec3(0.95, 0.77, 0.83);
    // vec3 albedo = vec3(242.0 / 255.0, 233.0 / 255.0, 206.0 / 255.0);

    vec3 accumColour = vec3(0.0);
    vec3 colorMask = vec3(1.0);
    // float firstDist = 0.0;

    for (int bounce; bounce<BOUNCES; bounce++) {
        //Trace ray
        vec2 tres = intersect(ro, rd);
        float t = tres.x;
        vec3 albedo = calcColour(tres.y);
        if (t < 0.0) {
            if (bounce == 0) return mix( 0.05*vec3(0.9,1.0,1.0), skyCol, smoothstep(0.1,0.25,rd.y) ); //Sky colour
            break;
        }

        vec3 pos = ro + rd * t;
        vec3 nor = calcNormal(pos);

        // if (bounce == 0)
        //     firstDist = t;

        colorMask *= albedo;

        vec3 iColor = vec3(0.0);

        float sunDif =  max(0.0, dot(sunDir, nor));
        float sunSha = 1.0; if (sunDif > EPSILON * 0.1) sunSha = shadow(pos + nor * EPSILON, sunDir);
        iColor += sunCol * sunDif * sunSha;
        //TODO: Specular

        // vec3 skyPoint = cosineDirection( sa + 7.1*float(iSample) + 5681.123 + float(bounce)*92.13, nor);
        sa += 7.1*float(iSample) + 5681.123 + float(bounce)*92.13;
        vec3 skyPoint = cosWeightedRandomHemisphereDirection( sa, nor);
        float skySha = shadow( pos + nor*EPSILON, skyPoint);
        iColor += skyCol * skySha;

        accumColour += colorMask * iColor;

        sa = 76.2 + 73.1*float(bounce) + sa + 17.7*float(iSample);

        rd = cosineDirection(sa, nor); //This one is as good as the other one, and maybe faster
        // float xi1 = hash(sa);
        // hash(sa);
        // hash(sa);
        // float xi2 = hash(sa);
        // rd = sampleHemisphereCosWeighted(nor, xi1, xi2);
        ro = pos;
    }

    return accumColour;
}

mat3 setCamera( in vec3 ro, in vec3 rt, in float cr )
{
	vec3 cw = normalize(rt-ro);
	vec3 cp = vec3(sin(cr), cos(cr),0.0);
	vec3 cu = normalize( cross(cw,cp) );
	vec3 cv = normalize( cross(cu,cw) );
    return mat3( cu, cv, -cw );
}

vec3 trace(vec2 uv, float seed_in, vec2 dims, vec2 screen_coords) {
    float seed = seed_in;
    float sa = hash( seed );

    vec2 p = (-dims.xy + 2.0*screen_coords) / dims.y;
    p.y = -p.y; //Flip the y as love2d for some reason makes the result upside down compared to shadertoy

    //Temporary camera
    vec3 ro = vec3(0.0);
    vec3 ta = vec3(1.5, 0.7, 1.5);
    mat3 ca = setCamera(ro, ta, 0.0);
    vec3 rd = normalize(ca * vec3(p, -1.3));

    vec3 col = calculateLight(ro, rd, sa);

    return col;
}

void main() {
    vec2 dims = vec2(imageSize(img_output));
    vec3 pixel_coords = vec3(gl_GlobalInvocationID.xyz);
    vec2 uv = pixel_coords.xy / dims;

    float seed = fract(sin(dot(pixel_coords.xy*2.123, vec2(12.949850, 78.23834)))) + 1113.12370912874;

    // imageStore(img_output, pixel_coords.xy, vec4(uv, 0.0, 1.0));
    vec3 result = trace(uv, seed, dims, pixel_coords.xy);
    imageStore(img_output, ivec2(gl_GlobalInvocationID.xy), vec4(result, 1.0));
}
