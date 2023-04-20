use parry2d::math::{Point, Vector, Real};

pub enum InnOuter {
    Inner,
    Outer,
    Boundary,
}

/// 检查点是否在三角形范围内， 要求p1 p2 p3 要么顺时针要么逆时针
pub fn include_tri2(p: &Point<Real>, p1: &Point<Real>, p2: &Point<Real>, p3: &Point<Real>) -> InnOuter
{
    let zero = 0.0;
    let v1 = Vector::new(p.x - p1.x, p.y - p1.y);
    let v2 = Vector::new(p.x - p2.x, p.y - p2.y);
    let v3 = Vector::new(p.x - p3.x, p.y - p3.y);
    // 要求3次叉乘结果，要么都大于0 要么都小于0
    let r = cross(&v1, &v2);
    if r > zero {
        let r = cross(&v2,&v3);
        if r > zero {
        } else if r < zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v3, &v1);
        if r > zero {
            InnOuter::Inner
        } else if r < zero {
            InnOuter::Outer
        } else {
            InnOuter::Boundary
        }
    } else if r < zero {
        let r = cross(&v2, &v3);
        if r < zero {
        } else if r > zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v3, &v1);
        if r < zero {
            InnOuter::Inner
        } else if r > zero {
            InnOuter::Outer
        } else {
            InnOuter::Boundary
        }
    } else {
        InnOuter::Boundary
    }
}

pub fn cross(p1: &Vector<Real>, p2: &Vector<Real>) -> Real{
	(p1.x * p2.y) - (p1.y * p2.x)
}
/// 检查点是否在四边形范围内， 要求p1 p2 p3 p4 要么顺时针要么逆时针
pub fn include_quad2(
    p: &Point<Real>,
    p1: &Point<Real>,
    p2: &Point<Real>,
    p3: &Point<Real>,
    p4: &Point<Real>,
) -> InnOuter
{
    let zero = 0.0;
	// let min = <S as Float>::min_value();
	// 非三维向量叉乘恐慌， （优化：TODO）
    let v1 = Vector::new(p.x - p1.x, p.y - p1.y);
    let v2 = Vector::new(p.x - p2.x, p.y - p2.y);
    let v3 = Vector::new(p.x - p3.x, p.y - p3.y);
    let v4 = Vector::new(p.x - p4.x, p.y - p4.y);
    // 要求4次叉乘结果，要么都大于0 要么都小于0
    let r = cross(&v1,&v2);
    if r > zero {
        let r = cross(&v2, &v3);
        if r > zero {
        } else if r < zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v3, &v4);
        if r > zero {
        } else if r < zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v4, &v1);
        if r > zero {
            InnOuter::Inner
        } else if r < zero {
            InnOuter::Outer
        } else {
            InnOuter::Boundary
        }
    } else if r < zero {
        let r = cross(&v2, &v3);
        if r < zero {
        } else if r > zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v3, &v4);
        if r < zero {
        } else if r > zero {
            return InnOuter::Outer;
        } else {
            return InnOuter::Boundary;
        }
        let r = cross(&v4, &v1);
        if r < zero {
            InnOuter::Inner
        } else if r > zero {
            InnOuter::Outer
        } else {
            InnOuter::Boundary
        }
    } else {
        InnOuter::Boundary
    }
}
