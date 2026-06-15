#include <utilities/log.h>

#include "alphashapes2d-periodic.h"
#include <topology/filtration.h>
#include <topology/static-persistence.h>
#include <topology/persistence-diagram.h>
#include <iostream>

#include <fstream>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/serialization/map.hpp>

#include <boost/program_options.hpp>


typedef Filtration<AlphaSimplex2D>              AlphaFiltration;
typedef StaticPersistence<>                     Persistence;
typedef PersistenceDiagram<>                    PDgm;

namespace po = boost::program_options;

int main(int argc, char** argv) 
{
#ifdef LOGGING
    rlog::RLogInit(argc, argv);

    stdoutLog.subscribeTo( RLOG_CHANNEL("error") );
    stdoutLog.subscribeTo( RLOG_CHANNEL("info") );
    //stdoutLog.subscribeTo( RLOG_CHANNEL("topology/filtration") );
    //stdoutLog.subscribeTo( RLOG_CHANNEL("topology/cycle") );
#endif

    //SetFrequency(GetCounter("filtration/pair"), 10000);
    //SetTrigger(GetCounter("filtration/pair"), GetCounter(""));

    std::string     infilename, outfilename;
    
    po::options_description hidden("Hidden options");
    hidden.add_options()
    ("input-file",   po::value<std::string>(&infilename),     "Point set whose alpha shape filtration and persistence we want to compute")
    ("output-file",  po::value<std::string>(&outfilename),    "Where to write the collection of persistence diagrams");
    
    po::positional_options_description pos;
    pos.add("input-file", 1);
    pos.add("output-file", 2);
    
    po::options_description all; all.add(hidden);
    
    po::variables_map vm;
    po::store(po::command_line_parser(argc, argv).
              options(all).positional(pos).run(), vm);
    po::notify(vm);
    
    if (!vm.count("input-file") || !vm.count("output-file"))
    {
        std::cout << "Usage: " << argv[0] << " input-file output-file" << std::endl;
        std::cout << hidden << std::endl;
        return 1;
    }


    // Read in the point set and compute its Delaunay triangulation
    std::ifstream in(infilename.c_str());
    double x,y;
    
    
    // VR: The first two "points" from the input file must define the antipodal corners of the periodic domain.
    // The periodic domain must be a square.
    double a,b,c,d;
    in >> a >> b >> c >> d;

    Delaunay2D Dt(GT::Iso_rectangle_2(a,b,c,d));

        
    while(in)
    {
        in >> x >> y;
        // if (!in) break;
        Point p(x,y);
        Dt.insert(p);
    }
    
    // VR: Switch to 1-sheeted cover if possible
    if (Dt.is_triangulation_in_1_sheet())
        Dt.convert_to_1_sheeted_covering();
    else
        std::cout << "Periodic Triangulation not in one sheet. There may be problems ahead" << std::endl;
    
    rInfo("Delaunay triangulation computed");
   
    AlphaFiltration af;
    fill_complex(Dt, af);
    rInfo("Simplices: %i", af.size());

    // Create the alpha-shape filtration
    af.sort(AlphaSimplex2D::AlphaOrder());
    rInfo("Filtration initialized");

    Persistence p(af);
    rInfo("Persistence initializaed");

    p.pair_simplices();
    rInfo("Simplices paired");

    Persistence::SimplexMap<AlphaFiltration>    m       = p.make_simplex_map(af);
    std::map<Dimension, PDgm>                   dgms;
    init_diagrams(dgms, p.begin(), p.end(), 
                  evaluate_through_map(m, AlphaSimplex2D::AlphaValueEvaluator()),
                  evaluate_through_map(m, AlphaSimplex2D::DimensionExtractor()));

#if 0
    std::cout << 0 << std::endl << dgms[0] << std::endl;
    std::cout << 1 << std::endl << dgms[1] << std::endl;
#endif
    
    std::ofstream ofs(outfilename.c_str());
    boost::archive::binary_oarchive oa(ofs);
    oa << dgms;
}

