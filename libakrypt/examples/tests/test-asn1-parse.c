 #include <stdio.h>
 #include <stdlib.h>
 #include <string.h>
 #include <libakrypt.h>

/* ----------------------------------------------------------------------------------------------- */
 static ak_uint8 test_data[] = {
   0x30, 0x82, 0x05, 0x14, 0x30, 0x82, 0x04, 0xc1, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x10, 0x4e,
   0x6d, 0x47, 0x8b, 0x26, 0xf2, 0x7d, 0x65, 0x7f, 0x76, 0x8e, 0x02, 0x5c, 0xe3, 0xd3, 0x93, 0x30,
   0x0a, 0x06, 0x08, 0x2a, 0x85, 0x03, 0x07, 0x01, 0x01, 0x03, 0x02, 0x30, 0x82, 0x01, 0x24, 0x31,
   0x1e, 0x30, 0x1c, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x01, 0x16, 0x0f,
   0x64, 0x69, 0x74, 0x40, 0x6d, 0x69, 0x6e, 0x73, 0x76, 0x79, 0x61, 0x7a, 0x2e, 0x72, 0x75, 0x31,
   0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x52, 0x55, 0x31, 0x18, 0x30, 0x16,
   0x06, 0x03, 0x55, 0x04, 0x08, 0x0c, 0x0f, 0x37, 0x37, 0x20, 0xd0, 0x9c, 0xd0, 0xbe, 0xd1, 0x81,
   0xd0, 0xba, 0xd0, 0xb2, 0xd0, 0xb0, 0x31, 0x19, 0x30, 0x17, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0c,
   0x10, 0xd0, 0xb3, 0x2e, 0x20, 0xd0, 0x9c, 0xd0, 0xbe, 0xd1, 0x81, 0xd0, 0xba, 0xd0, 0xb2, 0xd0,
   0xb0, 0x31, 0x2e, 0x30, 0x2c, 0x06, 0x03, 0x55, 0x04, 0x09, 0x0c, 0x25, 0xd1, 0x83, 0xd0, 0xbb,
   0xd0, 0xb8, 0xd1, 0x86, 0xd0, 0xb0, 0x20, 0xd0, 0xa2, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80, 0xd1,
   0x81, 0xd0, 0xba, 0xd0, 0xb0, 0xd1, 0x8f, 0x2c, 0x20, 0xd0, 0xb4, 0xd0, 0xbe, 0xd0, 0xbc, 0x20,
   0x37, 0x31, 0x2c, 0x30, 0x2a, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c, 0x23, 0xd0, 0x9c, 0xd0, 0xb8,
   0xd0, 0xbd, 0xd0, 0xba, 0xd0, 0xbe, 0xd0, 0xbc, 0xd1, 0x81, 0xd0, 0xb2, 0xd1, 0x8f, 0xd0, 0xb7,
   0xd1, 0x8c, 0x20, 0xd0, 0xa0, 0xd0, 0xbe, 0xd1, 0x81, 0xd1, 0x81, 0xd0, 0xb8, 0xd0, 0xb8, 0x31,
   0x18, 0x30, 0x16, 0x06, 0x05, 0x2a, 0x85, 0x03, 0x64, 0x01, 0x12, 0x0d, 0x31, 0x30, 0x34, 0x37,
   0x37, 0x30, 0x32, 0x30, 0x32, 0x36, 0x37, 0x30, 0x31, 0x31, 0x1a, 0x30, 0x18, 0x06, 0x08, 0x2a,
   0x85, 0x03, 0x03, 0x81, 0x03, 0x01, 0x01, 0x12, 0x0c, 0x30, 0x30, 0x37, 0x37, 0x31, 0x30, 0x34,
   0x37, 0x34, 0x33, 0x37, 0x35, 0x31, 0x2c, 0x30, 0x2a, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x23,
   0xd0, 0x9c, 0xd0, 0xb8, 0xd0, 0xbd, 0xd0, 0xba, 0xd0, 0xbe, 0xd0, 0xbc, 0xd1, 0x81, 0xd0, 0xb2,
   0xd1, 0x8f, 0xd0, 0xb7, 0xd1, 0x8c, 0x20, 0xd0, 0xa0, 0xd0, 0xbe, 0xd1, 0x81, 0xd1, 0x81, 0xd0,
   0xb8, 0xd0, 0xb8, 0x30, 0x1e, 0x17, 0x0d, 0x31, 0x38, 0x30, 0x37, 0x30, 0x36, 0x31, 0x32, 0x31,
   0x38, 0x30, 0x36, 0x5a, 0x17, 0x0d, 0x33, 0x36, 0x30, 0x37, 0x30, 0x31, 0x31, 0x32, 0x31, 0x38,
   0x30, 0x36, 0x5a, 0x30, 0x82, 0x01, 0x24, 0x31, 0x1e, 0x30, 0x1c, 0x06, 0x09, 0x2a, 0x86, 0x48,
   0x86, 0xf7, 0x0d, 0x01, 0x09, 0x01, 0x16, 0x0f, 0x64, 0x69, 0x74, 0x40, 0x6d, 0x69, 0x6e, 0x73,
   0x76, 0x79, 0x61, 0x7a, 0x2e, 0x72, 0x75, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06,
   0x13, 0x02, 0x52, 0x55, 0x31, 0x18, 0x30, 0x16, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0c, 0x0f, 0x37,
   0x37, 0x20, 0xd0, 0x9c, 0xd0, 0xbe, 0xd1, 0x81, 0xd0, 0xba, 0xd0, 0xb2, 0xd0, 0xb0, 0x31, 0x19,
   0x30, 0x17, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0c, 0x10, 0xd0, 0xb3, 0x2e, 0x20, 0xd0, 0x9c, 0xd0,
   0xbe, 0xd1, 0x81, 0xd0, 0xba, 0xd0, 0xb2, 0xd0, 0xb0, 0x31, 0x2e, 0x30, 0x2c, 0x06, 0x03, 0x55,
   0x04, 0x09, 0x0c, 0x25, 0xd1, 0x83, 0xd0, 0xbb, 0xd0, 0xb8, 0xd1, 0x86, 0xd0, 0xb0, 0x20, 0xd0,
   0xa2, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80, 0xd1, 0x81, 0xd0, 0xba, 0xd0, 0xb0, 0xd1, 0x8f, 0x2c,
   0x20, 0xd0, 0xb4, 0xd0, 0xbe, 0xd0, 0xbc, 0x20, 0x37, 0x31, 0x2c, 0x30, 0x2a, 0x06, 0x03, 0x55,
   0x04, 0x0a, 0x0c, 0x23, 0xd0, 0x9c, 0xd0, 0xb8, 0xd0, 0xbd, 0xd0, 0xba, 0xd0, 0xbe, 0xd0, 0xbc,
   0xd1, 0x81, 0xd0, 0xb2, 0xd1, 0x8f, 0xd0, 0xb7, 0xd1, 0x8c, 0x20, 0xd0, 0xa0, 0xd0, 0xbe, 0xd1,
   0x81, 0xd1, 0x81, 0xd0, 0xb8, 0xd0, 0xb8, 0x31, 0x18, 0x30, 0x16, 0x06, 0x05, 0x2a, 0x85, 0x03,
   0x64, 0x01, 0x12, 0x0d, 0x31, 0x30, 0x34, 0x37, 0x37, 0x30, 0x32, 0x30, 0x32, 0x36, 0x37, 0x30,
   0x31, 0x31, 0x1a, 0x30, 0x18, 0x06, 0x08, 0x2a, 0x85, 0x03, 0x03, 0x81, 0x03, 0x01, 0x01, 0x12,
   0x0c, 0x30, 0x30, 0x37, 0x37, 0x31, 0x30, 0x34, 0x37, 0x34, 0x33, 0x37, 0x35, 0x31, 0x2c, 0x30,
   0x2a, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x23, 0xd0, 0x9c, 0xd0, 0xb8, 0xd0, 0xbd, 0xd0, 0xba,
   0xd0, 0xbe, 0xd0, 0xbc, 0xd1, 0x81, 0xd0, 0xb2, 0xd1, 0x8f, 0xd0, 0xb7, 0xd1, 0x8c, 0x20, 0xd0,
   0xa0, 0xd0, 0xbe, 0xd1, 0x81, 0xd1, 0x81, 0xd0, 0xb8, 0xd0, 0xb8, 0x30, 0x66, 0x30, 0x1f, 0x06,
   0x08, 0x2a, 0x85, 0x03, 0x07, 0x01, 0x01, 0x01, 0x01, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x85, 0x03,
   0x02, 0x02, 0x23, 0x01, 0x06, 0x08, 0x2a, 0x85, 0x03, 0x07, 0x01, 0x01, 0x02, 0x02, 0x03, 0x43,
   0x00, 0x04, 0x40, 0x75, 0x39, 0x2a, 0x45, 0xa7, 0xb9, 0xa2, 0x95, 0x7d, 0xf7, 0x10, 0xfd, 0x22,
   0x92, 0x07, 0xba, 0x1d, 0xb6, 0x5a, 0x71, 0x8a, 0x7d, 0x7d, 0x58, 0xfc, 0xb1, 0x46, 0xb9, 0x45,
   0x61, 0x57, 0xac, 0x1d, 0xbb, 0x48, 0xa5, 0xf9, 0x4a, 0xfb, 0x48, 0x19, 0xea, 0x6a, 0x29, 0xeb,
   0xfa, 0xf5, 0x14, 0x98, 0x78, 0x71, 0xca, 0x47, 0xe8, 0xd3, 0xf5, 0x85, 0xf6, 0x36, 0xe4, 0x8a,
   0xf7, 0x03, 0x8d, 0xa3, 0x82, 0x01, 0xc2, 0x30, 0x82, 0x01, 0xbe, 0x30, 0x81, 0xf5, 0x06, 0x05,
   0x2a, 0x85, 0x03, 0x64, 0x70, 0x04, 0x81, 0xeb, 0x30, 0x81, 0xe8, 0x0c, 0x34, 0xd0, 0x9f, 0xd0,
   0x90, 0xd0, 0x9a, 0xd0, 0x9c, 0x20, 0xc2, 0xab, 0xd0, 0x9a, 0xd1, 0x80, 0xd0, 0xb8, 0xd0, 0xbf,
   0xd1, 0x82, 0xd0, 0xbe, 0xd0, 0x9f, 0xd1, 0x80, 0xd0, 0xbe, 0x20, 0x48, 0x53, 0x4d, 0xc2, 0xbb,
   0x20, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80, 0xd1, 0x81, 0xd0, 0xb8, 0xd0, 0xb8, 0x20, 0x32, 0x2e,
   0x30, 0x0c, 0x43, 0xd0, 0x9f, 0xd0, 0x90, 0xd0, 0x9a, 0x20, 0xc2, 0xab, 0xd0, 0x93, 0xd0, 0xbe,
   0xd0, 0xbb, 0xd0, 0xbe, 0xd0, 0xb2, 0xd0, 0xbd, 0xd0, 0xbe, 0xd0, 0xb9, 0x20, 0xd1, 0x83, 0xd0,
   0xb4, 0xd0, 0xbe, 0xd1, 0x81, 0xd1, 0x82, 0xd0, 0xbe, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80, 0xd1,
   0x8f, 0xd1, 0x8e, 0xd1, 0x89, 0xd0, 0xb8, 0xd0, 0xb9, 0x20, 0xd1, 0x86, 0xd0, 0xb5, 0xd0, 0xbd,
   0xd1, 0x82, 0xd1, 0x80, 0xc2, 0xbb, 0x0c, 0x35, 0xd0, 0x97, 0xd0, 0xb0, 0xd0, 0xba, 0xd0, 0xbb,
   0xd1, 0x8e, 0xd1, 0x87, 0xd0, 0xb5, 0xd0, 0xbd, 0xd0, 0xb8, 0xd0, 0xb5, 0x20, 0xe2, 0x84, 0x96,
   0x20, 0x31, 0x34, 0x39, 0x2f, 0x33, 0x2f, 0x32, 0x2f, 0x32, 0x2f, 0x32, 0x33, 0x20, 0xd0, 0xbe,
   0xd1, 0x82, 0x20, 0x30, 0x32, 0x2e, 0x30, 0x33, 0x2e, 0x32, 0x30, 0x31, 0x38, 0x0c, 0x34, 0xd0,
   0x97, 0xd0, 0xb0, 0xd0, 0xba, 0xd0, 0xbb, 0xd1, 0x8e, 0xd1, 0x87, 0xd0, 0xb5, 0xd0, 0xbd, 0xd0,
   0xb8, 0xd0, 0xb5, 0x20, 0xe2, 0x84, 0x96, 0x20, 0x31, 0x34, 0x39, 0x2f, 0x37, 0x2f, 0x36, 0x2f,
   0x31, 0x30, 0x35, 0x20, 0xd0, 0xbe, 0xd1, 0x82, 0x20, 0x32, 0x37, 0x2e, 0x30, 0x36, 0x2e, 0x32,
   0x30, 0x31, 0x38, 0x30, 0x3f, 0x06, 0x05, 0x2a, 0x85, 0x03, 0x64, 0x6f, 0x04, 0x36, 0x0c, 0x34,
   0xd0, 0x9f, 0xd0, 0x90, 0xd0, 0x9a, 0xd0, 0x9c, 0x20, 0xc2, 0xab, 0xd0, 0x9a, 0xd1, 0x80, 0xd0,
   0xb8, 0xd0, 0xbf, 0xd1, 0x82, 0xd0, 0xbe, 0xd0, 0x9f, 0xd1, 0x80, 0xd0, 0xbe, 0x20, 0x48, 0x53,
   0x4d, 0xc2, 0xbb, 0x20, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80, 0xd1, 0x81, 0xd0, 0xb8, 0xd0, 0xb8,
   0x20, 0x32, 0x2e, 0x30, 0x30, 0x43, 0x06, 0x03, 0x55, 0x1d, 0x20, 0x04, 0x3c, 0x30, 0x3a, 0x30,
   0x08, 0x06, 0x06, 0x2a, 0x85, 0x03, 0x64, 0x71, 0x01, 0x30, 0x08, 0x06, 0x06, 0x2a, 0x85, 0x03,
   0x64, 0x71, 0x02, 0x30, 0x08, 0x06, 0x06, 0x2a, 0x85, 0x03, 0x64, 0x71, 0x03, 0x30, 0x08, 0x06,
   0x06, 0x2a, 0x85, 0x03, 0x64, 0x71, 0x04, 0x30, 0x08, 0x06, 0x06, 0x2a, 0x85, 0x03, 0x64, 0x71,
   0x05, 0x30, 0x06, 0x06, 0x04, 0x55, 0x1d, 0x20, 0x00, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x1d, 0x0f,
   0x01, 0x01, 0xff, 0x04, 0x04, 0x03, 0x02, 0x01, 0x06, 0x30, 0x0f, 0x06, 0x03, 0x55, 0x1d, 0x13,
   0x01, 0x01, 0xff, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xff, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x1d,
   0x0e, 0x04, 0x16, 0x04, 0x14, 0xc2, 0x54, 0xf1, 0xb4, 0x6b, 0xd4, 0x4c, 0xb7, 0xe0, 0x6d, 0x36,
   0xb4, 0x23, 0x90, 0xf1, 0xfe, 0xc3, 0x3c, 0x9b, 0x06, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x85, 0x03,
   0x07, 0x01, 0x01, 0x03, 0x02, 0x03, 0x41, 0x00, 0x9a, 0xfa, 0xfd, 0xe2, 0x3b, 0xac, 0x72, 0xfb,
   0xf8, 0x5b, 0x10, 0x9e, 0x81, 0xf6, 0x8b, 0xa0, 0xd5, 0xc6, 0xa6, 0xa5, 0x6c, 0x8c, 0x4b, 0x2a,
   0x3d, 0x39, 0x79, 0xda, 0x59, 0x18, 0xf2, 0xcb, 0x6f, 0xa0, 0x76, 0x3d, 0x30, 0x0c, 0xc9, 0xae,
   0xe9, 0x4a, 0xdf, 0x61, 0x6f, 0xc4, 0x27, 0x14, 0x00, 0x60, 0xb1, 0x1e, 0x08, 0x13, 0x98, 0x13,
   0xe1, 0x55, 0x64, 0x0d, 0x66, 0xd7, 0xfe, 0x7e
 };

/* ----------------------------------------------------------------------------------------------- */
 int main( int argc, char *argv[] )
{
  ak_asn1 asn = NULL;

 /* инициализируем библиотеку */
  if( ak_libakrypt_create( ak_function_log_stderr ) != ak_true )
    return ak_libakrypt_destroy();

 /* если файл задан, то ОК */
  if( argc > 1 )
    ak_libakrypt_print_asn1( argv[1] );
   else
    { /* иначе декодируем константные данные */
       ak_asn1_decode( asn = ak_asn1_new( ),
                                test_data, sizeof( test_data ), ak_false );
       ak_asn1_print( asn );
       ak_asn1_delete( asn );
    }

  ak_libakrypt_destroy();
 return EXIT_SUCCESS;
}
