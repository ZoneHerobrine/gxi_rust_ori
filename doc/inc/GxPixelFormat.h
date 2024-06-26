/**
@File      GxPixelFormat.h
@Brief     the pixel format enumeration type for the GxIAPI dll module. 
@Author    Software Department
@Date      2022-11-20
@Version   1.0.2211.9201
*/

#ifndef GX_PIXEL_FORMAT_H
#define GX_PIXEL_FORMAT_H

//------------------------------------------------------------------------------------
typedef enum GX_PIXEL_SIZE_ENTRY
{
	GX_PIXEL_SIZE_BPP8  = 8,
	GX_PIXEL_SIZE_BPP10 = 10,
	GX_PIXEL_SIZE_BPP12 = 12,
	GX_PIXEL_SIZE_BPP16 = 16,
	GX_PIXEL_SIZE_BPP24 = 24,
	GX_PIXEL_SIZE_BPP30 = 30,
	GX_PIXEL_SIZE_BPP32 = 32,
	GX_PIXEL_SIZE_BPP36 = 36,
	GX_PIXEL_SIZE_BPP48 = 48,
	GX_PIXEL_SIZE_BPP64 = 64,
}GX_PIXEL_SIZE_ENTRY;

typedef enum GX_PIXEL_COLOR_FILTER_ENTRY
{
	GX_COLOR_FILTER_NONE       = 0,                        ///<None
	GX_COLOR_FILTER_BAYER_RG = 1,                        ///<RG format
	GX_COLOR_FILTER_BAYER_GB = 2,                        ///<GB format
	GX_COLOR_FILTER_BAYER_GR = 3,                        ///<GR format
	GX_COLOR_FILTER_BAYER_BG = 4,                        ///<BG format
}GX_PIXEL_COLOR_FILTER_ENTRY;

#define GX_PIXEL_MONO                  ( 0x01000000 )
#define GX_PIXEL_COLOR                 ( 0x02000000 )

#define GX_PIXEL_8BIT                  ( 0x00080000 )
#define GX_PIXEL_10BIT                 ( 0x000A0000 )
#define GX_PIXEL_12BIT                 ( 0x000C0000 )
#define GX_PIXEL_16BIT                 ( 0x00100000 )
#define GX_PIXEL_24BIT                 ( 0x00180000 )
#define GX_PIXEL_30BIT                 ( 0x001E0000 )
#define GX_PIXEL_32BIT                 ( 0x00200000 )
#define GX_PIXEL_36BIT                 ( 0x00240000 )
#define GX_PIXEL_48BIT                 ( 0x00300000 )
#define GX_PIXEL_64BIT                 ( 0x00400000 )

#ifndef GX_PIXEL_FORMAT_ENTRY_DEFINED
#define	GX_PIXEL_FORMAT_ENTRY_DEFINED
typedef enum 

{
    GX_PIXEL_FORMAT_UNDEFINED          = (0),
    GX_PIXEL_FORMAT_MONO8              = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x0001),//0x1080001,
    GX_PIXEL_FORMAT_MONO8_SIGNED       = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x0002),//0x1080002,
    GX_PIXEL_FORMAT_MONO10             = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0003),//0x1100003,	
    GX_PIXEL_FORMAT_MONO12             = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0005),//0x1100005,	
    GX_PIXEL_FORMAT_MONO14             = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0025),//0x1100025,
    GX_PIXEL_FORMAT_MONO16             = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0007),//0x1100007,

    GX_PIXEL_FORMAT_BAYER_GR8          = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x0008),//0x1080008,               
    GX_PIXEL_FORMAT_BAYER_RG8          = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x0009),//0x1080009,                
    GX_PIXEL_FORMAT_BAYER_GB8          = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x000A),//0x108000A,
    GX_PIXEL_FORMAT_BAYER_BG8          = (GX_PIXEL_MONO  | GX_PIXEL_8BIT  | 0x000B),//0x108000B,

    GX_PIXEL_FORMAT_BAYER_GR10         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x000C),//0x110000C,                
    GX_PIXEL_FORMAT_BAYER_RG10         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x000D),//0x110000D,
    GX_PIXEL_FORMAT_BAYER_GB10         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x000E),//0x110000E,
    GX_PIXEL_FORMAT_BAYER_BG10         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x000F),//0x110000F,

    GX_PIXEL_FORMAT_BAYER_GR12         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0010),//0x1100010,              
    GX_PIXEL_FORMAT_BAYER_RG12         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0011),//0x1100011,
    GX_PIXEL_FORMAT_BAYER_GB12         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0012),//0x1100012,
    GX_PIXEL_FORMAT_BAYER_BG12         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0013),//0x1100013,

    GX_PIXEL_FORMAT_BAYER_GR16         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x002E),//0x110002E,                
    GX_PIXEL_FORMAT_BAYER_RG16         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x002F),//0x110002F,
    GX_PIXEL_FORMAT_BAYER_GB16         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0030),//0x1100030,
    GX_PIXEL_FORMAT_BAYER_BG16         = (GX_PIXEL_MONO  | GX_PIXEL_16BIT | 0x0031),//0x1100031,

    GX_PIXEL_FORMAT_RGB8_PLANAR        = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x0021),//0x2180021,
    GX_PIXEL_FORMAT_RGB10_PLANAR       = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0022),//0x2300022,
    GX_PIXEL_FORMAT_RGB12_PLANAR       = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0023),//0x2300023,
    GX_PIXEL_FORMAT_RGB16_PLANAR       = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0024),//0x2300024,

    GX_PIXEL_FORMAT_RGB8               = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x0014),//0x2180014,
    GX_PIXEL_FORMAT_RGB10              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0018),//0x2300018,
    GX_PIXEL_FORMAT_RGB12              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x001A),//0x230001A,
    GX_PIXEL_FORMAT_RGB14              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x005E),//0x230005E,
    GX_PIXEL_FORMAT_RGB16              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0033),//0x2300033,

    GX_PIXEL_FORMAT_BGR8               = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x0015),//0x2180015,
    GX_PIXEL_FORMAT_BGR10              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x0019),//0x2300019,
    GX_PIXEL_FORMAT_BGR12              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x001B),//0x230001B,
    GX_PIXEL_FORMAT_BGR14              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x004A),//0x230004A,
    GX_PIXEL_FORMAT_BGR16              = (GX_PIXEL_COLOR | GX_PIXEL_48BIT | 0x004B),//0x230004B,

    GX_PIXEL_FORMAT_RGBA8              = (GX_PIXEL_COLOR | GX_PIXEL_32BIT | 0x0016),//0x2200016,
    GX_PIXEL_FORMAT_BGRA8              = (GX_PIXEL_COLOR | GX_PIXEL_32BIT | 0x0017),//0x2200017,
    GX_PIXEL_FORMAT_ARGB8              = (GX_PIXEL_COLOR | GX_PIXEL_32BIT | 0x0018),//0x2200018,Not defined in standard protocol
    GX_PIXEL_FORMAT_ABGR8              = (GX_PIXEL_COLOR | GX_PIXEL_32BIT | 0x0019),//0x2200019,Not defined in standard protocol

    GX_PIXEL_FORMAT_YUV444_8           = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x0020),//0x2180020,
    GX_PIXEL_FORMAT_YUV422_8           = (GX_PIXEL_COLOR | GX_PIXEL_16BIT | 0x0032),//0x2100032,
    GX_PIXEL_FORMAT_YUV411_8           = (GX_PIXEL_COLOR | GX_PIXEL_12BIT | 0x001E),//0x20C001E,
    GX_PIXEL_FORMAT_YUV420_8_PLANAR    = (GX_PIXEL_COLOR | GX_PIXEL_12BIT | 0x0040),//0x20C0040,Not defined in standard protocol

    GX_PIXEL_FORMAT_YCBCR444_8         = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x005B),//0x218005B,
    GX_PIXEL_FORMAT_YCBCR422_8         = (GX_PIXEL_COLOR | GX_PIXEL_16BIT | 0x003B),//0x210003B,
    GX_PIXEL_FORMAT_YCBCR411_8         = (GX_PIXEL_COLOR | GX_PIXEL_12BIT | 0x005A),//0x20C005A,

    GX_PIXEL_FORMAT_YCBCR601_444_8     = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x003D),//0x218003D,
    GX_PIXEL_FORMAT_YCBCR601_422_8     = (GX_PIXEL_COLOR | GX_PIXEL_16BIT | 0x003E),//0x210003E,
    GX_PIXEL_FORMAT_YCBCR601_411_8     = (GX_PIXEL_COLOR | GX_PIXEL_12BIT | 0x003F),//0x20C003F,

    GX_PIXEL_FORMAT_YCBCR709_444_8     = (GX_PIXEL_COLOR | GX_PIXEL_24BIT | 0x0040),//0x2180040,
    GX_PIXEL_FORMAT_YCBCR709_422_8     = (GX_PIXEL_COLOR | GX_PIXEL_16BIT | 0x0041),//0x2100041,
    GX_PIXEL_FORMAT_YCBCR709_411_8     = (GX_PIXEL_COLOR | GX_PIXEL_12BIT | 0x0042),//0x20C0042,

	GX_PIXEL_FORMAT_MONO10_PACKED      = (GX_PIXEL_MONO  | GX_PIXEL_12BIT | 0x0004),//0x010C0004,GigE Vision specific format
	GX_PIXEL_FORMAT_MONO12_PACKED      = (GX_PIXEL_MONO  | GX_PIXEL_12BIT | 0x0006),//0x010C0006,GigE Vision specific format

}GX_PIXEL_FORMAT_ENTRY;
#endif
#endif //_GX_PIXEL_FORMAT_H_