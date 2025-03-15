//-----------------------------------------------------------------------------
//  (c) 2016-2017 by Basler AG
//  Project: GenApi
//  Author:  hn
//
//  License: This file is published under the license of the EMVA GenICam  Standard Group.
//  A text file describing the legal terms is included in  your installation as 'GenICam_license.pdf'.
//  If for some reason you are missing  this file please contact the EMVA or visit the website
//  (http://www.genicam.org) for a full copy.
//
//  THIS SOFTWARE IS PROVIDED BY THE EMVA GENICAM STANDARD GROUP "AS IS"
//  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
//  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
//  PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE EMVA GENICAM STANDARD  GROUP
//  OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,  SPECIAL,
//  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT  LIMITED TO,
//  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,  DATA, OR PROFITS;
//  OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY  THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT  (INCLUDING NEGLIGENCE OR OTHERWISE)
//  ARISING IN ANY WAY OUT OF THE USE  OF THIS SOFTWARE, EVEN IF ADVISED OF THE
//  POSSIBILITY OF SUCH DAMAGE.
//-----------------------------------------------------------------------------
/*!
\file
\brief    Declaration of CFirmwareUpdaterEx
*/
#ifndef GENFW_FIRMWAREUPDATEREX_H
#define GENFW_FIRMWAREUPDATEREX_H
#include <GenICamFwd.h>
#include <Base/GCException.h>
#include <FirmwareUpdate/FirmwareUpdater.h>
#include <Log/CLog.h>

namespace GENAPI_NAMESPACE 
{
// ----------------------------------------------------------------------------
//  Extension for testing firmware updater
    class GENFW_DECL CFirmwareUpdaterEx : public CFirmwareUpdater
    {
    public:

        /**
        \brief Runs the firmware update process for a device and logs result of verification.
        \param[in]  pDeviceToUpdate  The device to update wrapped by an object implementing IFirmwareUpdatableDevice.
        \param[in]  FirmwareUpdateInfo  The firmware update info object representing a firmware update to apply.
        \param[in]  pProgressObserver  A progress listener implementing pProgressObserver.
        \param[in]  pPreferredLanguageCode  The preferred ISO 639-1 language code.
        Falls back, if not available, to: first "en", then no language code, then the first found entry.

        \see ApplyUpdate for details

        */
        void ApplyUpdateVerify(
            IFirmwareUpdatableDevice* pDeviceToUpdate,
            const CFirmwareUpdateInfo& FirmwareUpdateInfo,
            IFirmwareUpdateProgressObserver* pProgressObserver = NULL,
            const char* pPreferredLanguageCode = NULL)
        {
            try
            {
                ApplyUpdate(pDeviceToUpdate, FirmwareUpdateInfo, pProgressObserver, pPreferredLanguageCode);
            }
            catch (GENICAM_NAMESPACE::LogicalErrorException &e)
            {
                GCLOGERROR(&GENICAM_NAMESPACE::CLog::GetLogger("CppUnit"), "Logical error exception occurred calling ApplyUpdate:%s", e.what());

                throw;
            }
            catch (...)
            {
                GCLOGERROR(&GENICAM_NAMESPACE::CLog::GetLogger("CppUnit"), "Unknown exception occurred calling ApplyUpdate");

                throw;
            }

        }

    };
}
#endif //GENFW_FIRMWAREUPDATEREX_H
