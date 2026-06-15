#include <utilities/log.h>
#include <boost/foreach.hpp>

AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Vertex& v): alpha_(0), attached_(false)
{
    for (int i = 0; i < 4; ++i)
        if (v.cell()->vertex(i)->point() == v.point())
            Parent::add(v.cell()->vertex(i));
}
// VR:  I'm not sure why there is the extra step of checking the cell that the vertex belongs to
// It's possible that in the periodic version that the point locations might be different, i.e. differ by an offset.
// Or perhaps more problematically, that the same point might appear twice in a single cell (but with different offsets)


AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Edge& e)
{
    Cell_handle c = e.first;
    Parent::add(c->vertex(e.second));
    Parent::add(c->vertex(e.third));
}

AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Edge& e, const SimplexSet& simplices, const Delaunay3D& Dt, Facet_circulator facet_bg,  const RealValue r)
{
    Cell_handle c = e.first;
    Parent::add(c->vertex(e.second));
    Parent::add(c->vertex(e.third));

    Facet_circulator cur = facet_bg;
    // VR: Don't need to worry about infinite vertex in a periodic triangulation
    //    while (Dt.is_infinite(*cur))    ++cur;
    SimplexSet::const_iterator cur_iter = simplices.find(AlphaSimplex3D(*cur));
    RealValue min = cur_iter->alpha();

    //VR: I've rewritten this function substantially to make use of the CGAL is_Gabriel function
    if (Dt.is_Gabriel(e)){
        attached_ = false;
        alpha_ = r;
    }
    else {
        attached_ = true;
        // VR: now we just have to figure out the smallest circumradius of the facets adjacent to the given edge
        if (facet_bg != 0) do
        {
            cur_iter = simplices.find(AlphaSimplex3D(*cur));
            RealValue val = cur_iter->alpha();
            if (val < min)
                min = val;
            ++cur;
        } while (cur != facet_bg);
        alpha_ = min;
    }
}

AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Facet& f)
{
    Cell_handle c = f.first;
    for (int i = 0; i < 4; ++i)
        if (i != f.second)
            Parent::add(c->vertex(i));
}

/*
AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Facet& f, const SimplexSet& simplices, const Delaunay3D& Dt)
{
    Cell_handle c = f.first;
    for (int i = 0; i < 4; ++i)
        if (i != f.second)
            Parent::add(c->vertex(i));

    Cell_handle o = c->neighbor(f.second);
    int oi = o->index(c);

    VertexSet::const_iterator v = static_cast<const Parent*>(this)->vertices().begin();
    const Point& p1 = (*v)->point();
    const Offset& o1 = (*v++)->offset();
    const Point& p2 = (*v)->point();
    const Offset& o2 = (*v++)->offset();
    const Point& p3 = (*v)->point();
    const Offset& o3 = (*v)->offset();

    //VR: I've rewritten this function substantially to make use of the CGAL is_Gabriel function
    if (Dt.is_Gabriel(f)){
        attached_ = false;
        //alpha_ = CGAL::squared_radius(p1+o1, p2+o2, p3+o3);
        alpha_ = CGAL::squared_radius(p1, p2, p3);
    }
    else {
        attached_ = true;
        alpha_ = std::min(simplices.find(AlphaSimplex3D(*c))->alpha(),
                          simplices.find(AlphaSimplex3D(*o))->alpha());
    }
}
*/

AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Facet& f, const SimplexSet& simplices, const Delaunay3D& Dt, const RealValue r)
{
    Cell_handle c = f.first;
    for (int i = 0; i < 4; ++i)
        if (i != f.second)
            Parent::add(c->vertex(i));
    
    Cell_handle o = c->neighbor(f.second);
    int oi = o->index(c);
    
    //VR: I've rewritten this function substantially to make use of the CGAL is_Gabriel function
    if (Dt.is_Gabriel(f)){
        attached_ = false;
        //alpha_ = CGAL::squared_radius(p1+o1, p2+o2, p3+o3);
        alpha_ = r;
    }
    else {
        attached_ = true;
        alpha_ = std::min(simplices.find(AlphaSimplex3D(*c))->alpha(),
                          simplices.find(AlphaSimplex3D(*o))->alpha());
    }
}


/*
AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Cell& c): attached_(false)
{
    for (int i = 0; i < 4; ++i)
        Parent::add(c.vertex(i));
        
    VertexSet::const_iterator v = static_cast<const Parent*>(this)->vertices().begin();
    Point p1 = (*v)->point();
    Offset o1 = (*v++)->offset();
    Point p2 = (*v)->point();
    Offset o2 = (*v++)->offset();
    Point p3 = (*v)->point();
    Offset o3 = (*v++)->offset();
    Point p4 = (*v)->point();
    Offset o4 = (*v)->offset();
    alpha_ = CGAL::squared_radius(p1+o1, p2+o2, p3+o3, p4+o4);
}
*/

/*
 AlphaSimplex3D::
 AlphaSimplex3D(const Delaunay3D::Cell& c): attached_(false)
 {
 for (int i = 0; i < 4; ++i)
 Parent::add(c.vertex(i));
 
 Point p1 = c.vertex(0)->point();
 int o1 = c.offset(0);
 Point p2 = c.vertex(1)->point();
 int o2 = c.offset(1);
 Point p3 = c.vertex(2)->point();
 int o3 = c.offset(2);
 Point p4 = c.vertex(3)->point();
 int o4 = c.offset(3);
 // can't do the following because the offsets aren't the right type
 //alpha_ = CGAL::squared_radius(p1+o1, p2+o2, p3+o3, p4+o4);
 }
*/


/* Okay so here's the problem with the above code:
 The vertex base class has offsets defined as triplets of integers (using the Offset type) but
 the cell base class returns an offset defined as a single integer that is manipulated in a fancy way (it seems). 
 
 If I access the vertex and its offset (as commented out above) I get a null offset, because we're in the 1-sheeted cover, and the vertex is always inside the original domain.
 To get the proper geometry of the cell, I should access the vertex offsets stored with the cell.  
 I can do this but the offsets are returned as a single integer.   So I guess I would need to insert the CGAL code to convert these integers into triples.
 
 I also tried to use the higher-level geometric access functions but wasn't accessing them in the right way and got a seg fault.
 The problem here is that I pass a Cell& to the AlphaSimplex3D function, but then need to use a Cell_handle to get the info I want. I don't know how to convert between the two. 
 
 Hooray! Problem fixed by putting the geometric calculations into the fill_simplex_set function instead of the AlphaSimplex3D creation function.
*/



AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Cell& c)
{
    for (int i = 0; i < 4; ++i)
        Parent::add(c.vertex(i));
}

AlphaSimplex3D::
AlphaSimplex3D(const Delaunay3D::Cell& c, const RealValue r): attached_(false)
{
    for (int i = 0; i < 4; ++i)
        Parent::add(c.vertex(i));
    
    alpha_ = r;
}



bool
AlphaSimplex3D::AlphaOrder::
operator()(const AlphaSimplex3D& first, const AlphaSimplex3D& second) const
{
    if (first.alpha() == second.alpha())
        return (first.dimension() < second.dimension());
    else
        return (first.alpha() < second.alpha());
}

std::ostream&
AlphaSimplex3D::
operator<<(std::ostream& out) const
{
    for (VertexSet::const_iterator cur = Parent::vertices().begin(); cur != Parent::vertices().end(); ++cur)
        out << **cur << ", ";
    out << "value = " << value();

    return out;
}

void fill_simplex_set(const Delaunay3D& Dt, AlphaSimplex3D::SimplexSet& simplices)
{
   // Compute all simplices with their alpha values and attachment information
   for(Cell_iterator cur = Dt.cells_begin(); cur != Dt.cells_end(); ++cur){
       Delaunay3D::Periodic_tetrahedron ptet = Dt.periodic_tetrahedron(cur);
#if CGAL_VERSION_NR >= CGAL_VERSION_NUMBER(4,11,0)
       Delaunay3D::Tetrahedron tet = Dt.construct_tetrahedron(ptet);
#else
       Delaunay3D::Tetrahedron tet = Dt.tetrahedron(ptet);
#endif
       RealValue sqrad = CGAL::squared_radius(tet[0], tet[1], tet[2], tet[3]);
       simplices.insert(AlphaSimplex3D(*cur, sqrad));
   }
   rInfo("Cells inserted");
   for(Facet_iterator cur = Dt.facets_begin(); cur != Dt.facets_end(); ++cur){
       Delaunay3D::Periodic_triangle ptri = Dt.periodic_triangle(*cur);
#if CGAL_VERSION_NR >= CGAL_VERSION_NUMBER(4,11,0)
       Delaunay3D::Triangle tri = Dt.construct_triangle(ptri);
#else
       Delaunay3D::Triangle tri = Dt.triangle(ptri);
#endif
       RealValue sqrad = CGAL::squared_radius(tri[0], tri[1], tri[2]);
       simplices.insert(AlphaSimplex3D(*cur, simplices, Dt, sqrad));
   }
   rInfo("Facets inserted");
   for(Edge_iterator cur = Dt.edges_begin(); cur != Dt.edges_end(); ++cur){
       Delaunay3D::Periodic_segment pseg = Dt.periodic_segment(*cur);
#if CGAL_VERSION_NR >= CGAL_VERSION_NUMBER(4,11,0)
       Delaunay3D::Segment seg = Dt.construct_segment(pseg);
#else
       Delaunay3D::Segment seg = Dt.segment(pseg);
#endif
       RealValue sqrad = CGAL::squared_radius(seg[0],seg[1]);
       simplices.insert(AlphaSimplex3D(*cur, simplices, Dt, Dt.incident_facets(*cur), sqrad));
   }
   rInfo("Edges inserted");
   for(Vertex_iterator cur = Dt.vertices_begin(); cur != Dt.vertices_end(); ++cur)
       simplices.insert(AlphaSimplex3D(*cur));
   rInfo("Vertices inserted");
}



template<class Filtration>
void fill_complex(const Delaunay3D& Dt, Filtration& filtration)
{
    AlphaSimplex3D::SimplexSet simplices;
    fill_simplex_set(Dt, simplices);
    BOOST_FOREACH(const AlphaSimplex3D& s, simplices)
        filtration.push_back(s);
}

