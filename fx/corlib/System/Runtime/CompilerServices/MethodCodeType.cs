using System.Reflection;
 
namespace System.Runtime.CompilerServices
{
    public enum MethodCodeType
    {
        IL = 0x0000,
        Native = 0x0001,
        OPTIL = 0x0002,
        Runtime = 0x0003,
    }
}