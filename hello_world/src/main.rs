const G:f64 = 6.6743e-11;

struct Body {
    position:[f64;2], 
    velocity:[f64;2],
    acceleration:[f64;2],
    mass:f64,
}

impl Body {
    fn NewBody (position:[f64;2], velocity:[f64;2], acceleration:[f64;2], mass:f64) -> Self {
        Body {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            mass: mass,
        }
    }
    fn getDist (&mut self, other_body: &Body) -> f64 {
        let subx = other_body.position[0] - self.position[0];
        let suby = other_body.position[1] - self.position[1];
        println!("subx:{}, suby:{}", subx, suby);

        return ((subx.powf(2.0))+(suby.powf(2.0))).sqrt()
    }

    fn getsoi (&mut self, other_body: &Body, dist:f64) -> f64 {
        let mut M:f64 = 0.0;
        let mut m:f64 = 0.0;

        if self.mass > other_body.mass {
            M = self.mass;
            m = other_body.mass;
        } else {
            M = other_body.mass;
            m = self.mass
        }
        return dist * ((m/M).powf(2.0/5.0));
    }

    fn getg (&mut self , other_body: &Body, dist:f64) -> f64 {
        let soi = self.getsoi(other_body, dist);

        if dist != soi {
            return G*((self.mass*other_body.mass)/2.0);
        } else {
            return 0.0;
        }
    }

    fn get_acceleration (&mut self, other_body: &Body) {
        let dist = self.getDist(other_body);
        let g = self.getg(other_body, dist);
        println!("dist:{}",dist);
        println!("g:{}",g);
        let diffx = self.position[0] - other_body.position[0];
        let diffy = self.position[1] - other_body.position[1];
        let total = diffx + diffy;
        let chunk = g/total;
        
        self.acceleration[0] += chunk*diffx;
        self.acceleration[1] += chunk*diffy;

        println!("{:?}",self.acceleration);
    }
}

fn main() {
    let mut body1 = Body::NewBody([0.0, 0.0], [0.0, 0.0], [0.0, 0.0], 400000.0);
    let mut body2 = Body::NewBody([10.0, 15.0], [0.0, 0.0], [0.0, 0.0], 30.0);
    
    body1.get_acceleration(&body2);
}
