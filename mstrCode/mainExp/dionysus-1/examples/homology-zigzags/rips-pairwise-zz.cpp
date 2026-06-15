/***************************************************************************

    rips-pairwise: Rips filtration + persistent homology 
    Copyright (C) 2009-2012   Dmitriy Morozov

    Adapted from its original version ("rips-pairwise.cpp" in
    the Dionysus library) by Steve Oudot (2012).

    Changelog (2012-11-22):

       - the barcode is now output on a log scale and in a format that is 
         compatible with the Matlab layer of the PLEX 2.5 library,

       - the barcode representation is now by closed intervals instead of 
         half-open intervals.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

***************************************************************************/

#include <topology/rips.h>
#include <topology/filtration.h>
#include <topology/static-persistence.h>
#include <topology/dynamic-persistence.h>
#include <topology/persistence-diagram.h>

#include <geometry/l2distance.h>
#include <geometry/distances.h>

#include <utilities/containers.h>           // for BackInsertFunctor
#include <utilities/timer.h>

#include <vector>

#include <boost/program_options.hpp>


typedef         PairwiseDistances<PointContainer, L2Distance>           PairDistances;
typedef         PairDistances::DistanceType                             DistanceType;
typedef         PairDistances::IndexType                                Vertex;

typedef         Rips<PairDistances>                                     Generator;
typedef         Generator::Simplex                                      Smplx;
typedef         Filtration<Smplx>                                       Fltr;
// typedef         StaticPersistence<>                                     Persistence;
typedef         DynamicPersistenceChains<>                              Persistence;
typedef         PersistenceDiagram<>                                    PDgm;

typedef         std::vector<std::list<std::pair<double, double> > >     IntervalsVector;

// Forward declarations of auxilliary functions
void write_intervals(std::ostream& out, const IntervalsVector& intervals, int skeleton_dimension);
void            program_options(int argc, char* argv[], std::string& infilename, Dimension& skeleton, DistanceType& max_distance, std::string& diagram_name);

int main(int argc, char* argv[])
{
    Dimension               skeleton;
    DistanceType            max_distance;
    std::string             infilename, diagram_name;

    program_options(argc, argv, infilename, skeleton, max_distance, diagram_name);
    std::ofstream           diagram_out(diagram_name.c_str());
    std::cout << "Diagram:         " << diagram_name << std::endl;

    PointContainer          points;
    read_points(infilename, points);

    PairDistances           distances(points);
    Generator               rips(distances);
    Generator::Evaluator    size(distances);
    Fltr                    f;
    
    // Generate 2-skeleton of the Rips complex for epsilon = 50
    rips.generate(skeleton, max_distance, make_push_back_functor(f));
    std::cout << "# Generated complex of size: " << f.size() << std::endl;

    // Generate filtration with respect to distance and compute its persistence
    f.sort(Generator::Comparison(distances));

    Timer persistence_timer; persistence_timer.start();
    Persistence p(f);
    p.pair_simplices();
    persistence_timer.stop();

#if 1
    // Create intervals DS
    IntervalsVector intervals(skeleton);

    // Output cycles
    Persistence::SimplexMap<Fltr>   m = p.make_simplex_map(f);
    for (Persistence::iterator cur = p.begin(); cur != p.end(); ++cur)
    {
        const Persistence::Cycle& cycle = cur->cycle;

        if (!cur->sign())        // only negative simplices have non-empty cycles
        {
            Persistence::OrderIndex birth = cur->pair;      // the cycle that cur killed was born when we added birth (another simplex)

            const Smplx& b = m[birth];
            const Smplx& d = m[cur];
            
            // if (b.dimension() != 1) continue;
            // std::cout << "Pair: (" << size(b) << ", " << size(d) << ")" << std::endl;
            if (b.dimension() >= skeleton) continue;
            // diagram_out << b.dimension() << " " << size(b) << " " << size(d) << std::endl;
	    intervals[b.dimension()].push_back(std::pair<double,double>(size(b), size(d)));
        } else if (cur->unpaired())    // positive could be unpaired
        {
            const Smplx& b = m[cur];
            // if (b.dimension() != 1) continue;
            
            // std::cout << "Unpaired birth: " << size(b) << std::endl;
            // cycle = cur->chain;      // TODO
            if (b.dimension() >= skeleton) continue;
            // diagram_out << b.dimension() << " " << size(b) << " inf" << std::endl;
	    intervals[b.dimension()].push_back(std::pair<double,double>(size(b), -1));
        }

        // Iterate over the cycle
        // for (Persistence::Cycle::const_iterator si =  cycle.begin();
        //                                                          si != cycle.end();     ++si)
        // {
        //     const Smplx& s = m[*si];
        //     //std::cout << s.dimension() << std::endl;
        //     const Smplx::VertexContainer& vertices = s.vertices();          // std::vector<Vertex> where Vertex = Distances::IndexType
        //     AssertMsg(vertices.size() == s.dimension() + 1, "dimension of a simplex is one less than the number of its vertices");
        //     std::cout << vertices[0] << " " << vertices[1] << std::endl;
        // }
    }
#endif
    
    persistence_timer.check("# Persistence timer");

    std::cout << "Writing intervals...";
    // Note (hack): use epsilons[vertices.size()-2]/2 as minimal value for the log-scale intervals to avoid intervals starting at -infinity and thus a scaling effect
    write_intervals(diagram_out, intervals, skeleton);
    std::cout << " done." << std::endl;

}


const double LOG2 = std::log(2.0);
double log2(double x) {
  return std::log(x) / LOG2;
}

void write_intervals(std::ostream& out, const IntervalsVector& intervals, int skeleton_dimension) {
  out << "I = { ";
    for (int d = 0; d<skeleton_dimension; d++) {
      out << "[ ";
      for (std::list<std::pair<double,double> >::const_iterator pit = intervals[d].begin(); pit != intervals[d].end(); pit++) {
	if (pit->first == 0)
	  out << "[-Inf;";
        else
	  out << "[" << log2(pit->first) << ";";
	if (pit->second >= 0)
	  out << log2(pit->second) << "] ";
	else
	  out << "Inf] ";
      }
      // add dummy interval if intervals list is empty
      if (intervals[d].empty())
	out << "[0;0] ";
      out << "] ,";
    }
    out << "} " << std::endl;
} 


void        program_options(int argc, char* argv[], std::string& infilename, Dimension& skeleton, DistanceType& max_distance, std::string& diagram_name)
{
    namespace po = boost::program_options;

    po::options_description     hidden("Hidden options");
    hidden.add_options()
        ("input-file",          po::value<std::string>(&infilename),        "Point set whose Rips zigzag we want to compute");
    
    po::options_description visible("Allowed options", 100);
    visible.add_options()
        ("help,h",                                                                                  "produce help message")
        ("skeleton-dimsnion,s", po::value<Dimension>(&skeleton)->default_value(2),                  "Dimension of the Rips complex we want to compute")
        ("max-distance,m",      po::value<DistanceType>(&max_distance)->default_value(Infinity),    "Maximum value for the Rips complex construction")
        ("diagram,d",           po::value<std::string>(&diagram_name),                              "Filename where to output the persistence diagram");
#if LOGGING
    std::vector<std::string>    log_channels;
    visible.add_options()
        ("log,l",               po::value< std::vector<std::string> >(&log_channels),           "log channels to turn on (info, debug, etc)");
#endif

    po::positional_options_description pos;
    pos.add("input-file", 1);
    
    po::options_description all; all.add(visible).add(hidden);

    po::variables_map vm;
    po::store(po::command_line_parser(argc, argv).
                  options(all).positional(pos).run(), vm);
    po::notify(vm);

#if LOGGING
    for (std::vector<std::string>::const_iterator cur = log_channels.begin(); cur != log_channels.end(); ++cur)
        stderrLog.subscribeTo( RLOG_CHANNEL(cur->c_str()) );
#endif

    if (vm.count("help") || !vm.count("input-file"))
    { 
        std::cout << "Usage: " << argv[0] << " [options] input-file" << std::endl;
        std::cout << visible << std::endl; 
        std::abort();
    }
}
